use argvalues::ArgValues;
use error;
use loader;
use download;

use gfx;
use glutin;
use gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use glutin::{ElementState, MouseButton, GlContext};

use notify::{Watcher, RecursiveMode, DebouncedEvent, watcher};
use std::sync::mpsc::{channel, TryRecvError};
use std::path::Path;

use std::time::{Instant, Duration};

pub enum TextureId {
    ZERO,
    ONE,
    TWO,
    THREE,
}

type ColorFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "position",
    }

    pipeline pipe {
        // Vertex buffer
        vbuf: gfx::VertexBuffer<Vertex> = (),

        // Uniforms
        i_global_time: gfx::Global<f32> = "iGlobalTime",
        i_time: gfx::Global<f32> = "iTime",
        i_resolution: gfx::Global<[f32; 3]> = "iResolution",
        i_mouse: gfx::Global<[f32; 4]> = "iMouse",
        i_frame: gfx::Global<i32> = "iFrame",
        i_channel0: gfx::TextureSampler<[f32; 4]> = "iChannel0",
        i_channel1: gfx::TextureSampler<[f32; 4]> = "iChannel1",
        i_channel2: gfx::TextureSampler<[f32; 4]> = "iChannel2",
        i_channel3: gfx::TextureSampler<[f32; 4]> = "iChannel3",

        // Output color
        frag_color: gfx::RenderTarget<ColorFormat> = "fragColor",
    }
}

const SCREEN: [Vertex; 4] = [
    Vertex{pos: [ 1.0,  1.0]}, // Top right
    Vertex{pos: [-1.0,  1.0]}, // Top left
    Vertex{pos: [-1.0, -1.0]}, // Bottom left
    Vertex{pos: [ 1.0, -1.0]}, // Bottom right
];

const SCREEN_INDICES: [u16; 6] = [
    0, 1, 2,
    0, 2, 3,
];

const CLEAR_COLOR: [f32; 4] = [1.0; 4];

pub fn run(av: &ArgValues) -> error::Result<()> {
    let (mut width, mut height) = (av.width, av.height);

    // Load vertex and fragment shaders into byte buffers
    let vert_src_buf = loader::load_vertex_shader();
    let frag_src_buf = match av.getid {
        Some(ref id) => {
            let (_, shadercode) = download::download(id)?;

            // Don't run default shader if downloading (with no --run flag)
            if av.getid.is_some() && !av.andrun {
                return Ok(());
            }

            if av.andrun {
                loader::format_shader_src(&shadercode)
            } else {
                loader::load_fragment_shader(av)?
            }
        },
        None => {
            loader::load_fragment_shader(av)?
        }
    };
    let (vert_src_buf, frag_src_buf) = (vert_src_buf.as_slice(), frag_src_buf.as_slice());

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(250))
        .expect("couldn't initialise notify");

    let shader_basename = match av.shaderpath {
        None => None,
        Some(ref path) => {
            let path = Path::new(path);

            watcher.watch(path.parent().unwrap(), RecursiveMode::NonRecursive)
                .expect("couldn't register inotify watch");

            Some(path.file_name().unwrap())
        }
    };

    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title("shadertoy-rs")
        .with_dimensions(width as u32, height as u32);

    let (api, version) = (glutin::Api::OpenGl, (3, 2));

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(api, version))
        .with_vsync(true);

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);

    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let mut pso = factory.create_pipeline_simple(&vert_src_buf, &frag_src_buf, pipe::new()).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SCREEN, &SCREEN_INDICES[..]);

    // Load textures
    let texture0 = loader::load_texture(&TextureId::ZERO, &av.texture0path, &mut factory)?;
    let texture1 = loader::load_texture(&TextureId::ONE, &av.texture1path, &mut factory)?;
    let texture2 = loader::load_texture(&TextureId::TWO, &av.texture2path, &mut factory)?;
    let texture3 = loader::load_texture(&TextureId::THREE, &av.texture3path, &mut factory)?;

    let sampler = factory.create_sampler_linear();

    let mut data = pipe::Data {
        vbuf: vertex_buffer,

        i_global_time: 0.0,
        i_time: 0.0,
        i_resolution: [width, height, width/height],
        i_mouse: [0.0; 4],
        i_frame: -1,

        i_channel0: (texture0, sampler.clone()),
        i_channel1: (texture1, sampler.clone()),
        i_channel2: (texture2, sampler.clone()),
        i_channel3: (texture3, sampler.clone()),

        frag_color: main_color,
    };

    let mut last_mouse = ElementState::Released;
    let mut current_mouse = ElementState::Released;

    let (mut mx, mut my) = (0.0, 0.0);

    let mut xyzw = [0.0; 4];

    let mut start_time = Instant::now();
    let mut running = true;

    while running {
        let mut shader_modified = false;

        events_loop.poll_events(|event| {
            use glutin::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::Closed |
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => running = false,

                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::F5),
                            ..
                        },
                        ..
                    } => shader_modified = true,

                    WindowEvent::Resized(new_width, new_height) => {
                        gfx_window_glutin::update_views(&window, &mut data.frag_color, &mut main_depth);
                        window.resize(new_width, new_height);

                        width = new_width as f32;
                        height = new_height as f32;
                    },

                    WindowEvent::CursorMoved{position: (x,y), ..} => {
                        mx = x as f32;
                        my = height - y as f32; // Flip y-axis
                    },

                    WindowEvent::MouseInput{state, button, ..} => {
                        last_mouse = current_mouse;
                        if state == ElementState::Pressed && button == MouseButton::Left {
                            current_mouse = ElementState::Pressed;
                        } else {
                            current_mouse = ElementState::Released;
                        }
                    },

                    _ => (),
                }
            }
        });

        // notify handling
        shader_modified = shader_modified | {
            let mut have_events = false;
            let basename = shader_basename.unwrap();

            loop {
                match rx.try_recv() {
                    Err(TryRecvError::Empty) => break,

                    // we handle both create and write here because some text editors write
                    // the modified file to a tmpfile then move it 
                    Ok(DebouncedEvent::Create(ref path)) | Ok(DebouncedEvent::Write(ref path))
                        | Ok(DebouncedEvent::Rename(_, ref path))
                        if path.ends_with(basename) => have_events = true,

                    Ok(_ev) => {
                        // println!(" >> unhandled notify event: {:?}", _ev);
                    },

                    Err(TryRecvError::Disconnected) => {
                        println!(" !! watch disconnected");
                        break;
                    }
                }
            }

            have_events
        };

        // this is a while loop so we can "break;" out of it prematurely
        // in the event that we can't recompile the shader, this means that the
        // old shader just continues running, and then we dump the error to stdout.
        while shader_modified {
            // Reload fragment shader into byte buffer
            let frag_src_res = loader::load_fragment_shader(av);
            if frag_src_res.is_err() {
                println!("failed to load fragment shader");
                break;
            }
            let frag_src_res = frag_src_res.unwrap();
            let frag_src_buf = frag_src_res.as_slice();

            // Recreate pipeline
            let pso_res = factory
                .create_pipeline_simple(vert_src_buf, frag_src_buf, pipe::new());

            pso = match pso_res {
                Ok(pso) => pso,
                Err(e) => {
                    println!("failed to create pipeline: {:?}", e);
                    break;
                }
            };

            // Reset uniforms
            data.i_global_time = 0.0;
            data.i_time = 0.0;
            data.i_resolution = [width, height, width/height];
            data.i_mouse = [0.0; 4];
            data.i_frame = -1;

            start_time = Instant::now();

            break;
        }

        // Mouse
        if current_mouse == ElementState::Pressed {
            xyzw[0] = mx;
            xyzw[1] = my;
            if last_mouse == ElementState::Released {
                xyzw[2] = mx;
                xyzw[3] = my;
            }
        } else {
            xyzw[2] = 0.0;
            xyzw[3] = 0.0;
        }
        data.i_mouse = xyzw;

        // Elapsed time
        let elapsed = start_time.elapsed();
        let elapsed_ms = (elapsed.as_secs() * 1000) + u64::from(elapsed.subsec_nanos()/1_000_000);
        let elapsed_sec = (elapsed_ms as f32) / 1000.0;
        data.i_global_time = elapsed_sec;
        data.i_time = elapsed_sec;

        // Resolution
        data.i_resolution = [width, height, width/height];

        // Frame
        data.i_frame += 1;

        // Draw
        encoder.clear(&data.frag_color, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }

    Ok(())
}
