use argvalues::ArgValues;
use download;
use error;
use gfx;
use gfx::texture;
use gfx::texture::FilterMethod;
use gfx::Factory;
use loader;

use old_school_gfx_glutin_ext::*;

use gfx::{traits::FactoryExt, Device};
use glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glutin::event::{ElementState, MouseButton};

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, TryRecvError};

use std::time::{Duration, Instant};

pub enum TextureId {
    Zero,
    One,
    Two,
    Three,
}

type ColorFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "position",
    }

    pipeline pipe {
        // Vertex buffer.
        vbuf: gfx::VertexBuffer<Vertex> = (),

        // Uniforms.
        i_global_time: gfx::Global<f32> = "iGlobalTime",
        i_time: gfx::Global<f32> = "iTime",
        i_resolution: gfx::Global<[f32; 3]> = "iResolution",
        i_mouse: gfx::Global<[f32; 4]> = "iMouse",
        i_frame: gfx::Global<i32> = "iFrame",
        i_channel0: gfx::TextureSampler<[f32; 4]> = "iChannel0",
        i_channel1: gfx::TextureSampler<[f32; 4]> = "iChannel1",
        i_channel2: gfx::TextureSampler<[f32; 4]> = "iChannel2",
        i_channel3: gfx::TextureSampler<[f32; 4]> = "iChannel3",

        // Output color.
        frag_color: gfx::RenderTarget<ColorFormat> = "fragColor",
    }
}

const SCREEN: [Vertex; 4] = [
    Vertex { pos: [1.0, 1.0] },   // Top right.
    Vertex { pos: [-1.0, 1.0] },  // Top left.
    Vertex { pos: [-1.0, -1.0] }, // Bottom left.
    Vertex { pos: [1.0, -1.0] },  // Bottom right.
];

const SCREEN_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const CLEAR_COLOR: [f32; 4] = [1.0; 4];

pub fn run(av: ArgValues) -> error::Result<()> {
    let (mut width, mut height) = (av.width, av.height);

    // Load vertex and fragment shaders into byte buffers.
    let vert_src_buf = loader::load_vertex_shader();
    let frag_src_buf = match av.getid {
        Some(ref id) => {
            let (_, shadercode) = download::download(id)?;

            // Don't run default shader if downloading (with no --run flag).
            if av.getid.is_some() && !av.andrun {
                return Ok(());
            }

            if av.andrun {
                loader::format_shader_src(&shadercode)
            } else {
                loader::load_fragment_shader(&av)?
            }
        }
        None => loader::load_fragment_shader(&av)?,
    };

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(250)).expect("Could not initialise notify");

    let shader_basename = av.shaderpath.as_ref().map(|path| {
        let path = Path::new(&path);
        watcher
            .watch(path.parent().unwrap(), RecursiveMode::NonRecursive)
            .expect("Could not register inotify watch");
        path.file_name().unwrap().to_os_string()
    });

    let event_loop = EventLoop::new();

    let shader_name = av
        .getid
        .as_ref()
        .or(av.shaderpath.as_ref())
        .or(av.examplename.as_ref());
    let shader_title = shader_name.map(|name| format!("{} - shadertoy-rs", name));
    let default_title = "shadertoy-rs".to_string();

    let window_title = if av.title.is_some() {
        av.title.as_ref()
    } else if shader_title.is_some() {
        shader_title.as_ref()
    } else {
        Some(&default_title)
    };

    let window_config = WindowBuilder::new()
        .with_title(window_title.unwrap())
        .with_inner_size(glutin::dpi::PhysicalSize::new(width, height));

    let (window, mut device, mut factory, main_color, mut main_depth) =
        glutin::ContextBuilder::new()
            .with_gfx_color_depth::<ColorFormat, DepthFormat>()
            .build_windowed(window_config, &event_loop)
            .unwrap()
            .init_gfx::<ColorFormat, DepthFormat>();

    if av.force_srgb_off {
        unsafe {
            device.with_gl(|gl| {
                gl.Disable(gfx_gl::FRAMEBUFFER_SRGB);
            })
        }
    }

    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let mut pso = factory
        .create_pipeline_simple(&vert_src_buf, &frag_src_buf, pipe::new())
        .unwrap();

    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(&SCREEN, &SCREEN_INDICES[..]);

    // Load textures.
    let texture0 = loader::load_texture(&TextureId::Zero, &av.texture0path, &mut factory)?;
    let texture1 = loader::load_texture(&TextureId::One, &av.texture1path, &mut factory)?;
    let texture2 = loader::load_texture(&TextureId::Two, &av.texture2path, &mut factory)?;
    let texture3 = loader::load_texture(&TextureId::Three, &av.texture3path, &mut factory)?;

    let needs_mipmap =
        |mode: FilterMethod| mode != FilterMethod::Scale && mode != FilterMethod::Bilinear;

    // Generate mipmaps if needed.
    if needs_mipmap(av.filter0) {
        encoder.generate_mipmap(&texture0)
    };
    if needs_mipmap(av.filter1) {
        encoder.generate_mipmap(&texture1)
    };
    if needs_mipmap(av.filter2) {
        encoder.generate_mipmap(&texture2)
    };
    if needs_mipmap(av.filter3) {
        encoder.generate_mipmap(&texture3)
    };

    let mut data = pipe::Data {
        vbuf: vertex_buffer,

        i_global_time: 0.0,
        i_time: 0.0,
        i_resolution: [width, height, width / height],
        i_mouse: [0.0; 4],
        i_frame: -1,

        i_channel0: (
            texture0,
            factory.create_sampler(texture::SamplerInfo::new(av.filter0, av.wrap0)),
        ),
        i_channel1: (
            texture1,
            factory.create_sampler(texture::SamplerInfo::new(av.filter1, av.wrap1)),
        ),
        i_channel2: (
            texture2,
            factory.create_sampler(texture::SamplerInfo::new(av.filter2, av.wrap2)),
        ),
        i_channel3: (
            texture3,
            factory.create_sampler(texture::SamplerInfo::new(av.filter3, av.wrap3)),
        ),

        frag_color: main_color,
    };

    let mut last_mouse = ElementState::Released;
    let mut current_mouse = ElementState::Released;

    let (mut mx, mut my) = (0.0, 0.0);

    let mut xyzw = [0.0; 4];

    let mut start_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let mut shader_modified = false;

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::F5),
                            ..
                        },
                    ..
                } => shader_modified = true,

                WindowEvent::Resized(size) => {
                    window.update_gfx(&mut data.frag_color, &mut main_depth);
                    window.resize(size);

                    width = size.width as f32;
                    height = size.height as f32;
                }

                WindowEvent::CursorMoved {
                    position: cursor_position,
                    ..
                } => {
                    mx = cursor_position.x as f32;
                    my = height - cursor_position.y as f32; // Flip y-axis.
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    last_mouse = current_mouse;
                    if state == ElementState::Pressed && button == MouseButton::Left {
                        current_mouse = ElementState::Pressed;
                    } else {
                        current_mouse = ElementState::Released;
                    }
                }

                _ => (),
            }
        }
        // Notify handling.
        shader_modified |= match shader_basename.is_some() {
            false => false,
            true => {
                let mut have_events = false;
                let basename = &shader_basename;

                loop {
                    match rx.try_recv() {
                        Err(TryRecvError::Empty) => break,

                        // We handle both create and write here because some text editors write the
                        // modified file to a temp file before moving it.
                        Ok(DebouncedEvent::Create(ref path))
                        | Ok(DebouncedEvent::Write(ref path))
                        | Ok(DebouncedEvent::Rename(_, ref path))
                            if path.ends_with(basename.as_ref().unwrap().as_os_str()) =>
                        {
                            have_events = true
                        }

                        Ok(_ev) => {
                            // println!(" >> unhandled notify event: {:?}", _ev);
                        }

                        Err(TryRecvError::Disconnected) => {
                            println!("Watch disconnected");
                            break;
                        }
                    }
                }

                have_events
            }
        };

        // Attempt to reload the shader if it has been modified. In the event that the new version
        // does not load/compile properly, the old version will continue running.
        if shader_modified {
            // Reload fragment shader into byte buffer.
            match loader::load_fragment_shader(&av) {
                Ok(frag_src_res) => {
                    let frag_src_buf = frag_src_res.as_slice();

                    // Recreate pipeline.
                    match factory.create_pipeline_simple(&vert_src_buf, frag_src_buf, pipe::new()) {
                        Ok(new_pso) => {
                            pso = new_pso;

                            // Reset uniforms.
                            data.i_global_time = 0.0;
                            data.i_time = 0.0;
                            data.i_resolution = [width, height, width / height];
                            data.i_mouse = [0.0; 4];
                            data.i_frame = -1;

                            start_time = Instant::now();
                        }

                        Err(e) => println!("Failed to create pipeline: {:?}", e),
                    }
                }

                Err(e) => println!("Failed to load fragment shader: {:?}", e),
            }
        }

        // Mouse.
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

        // Elapsed time.
        let elapsed = start_time.elapsed();
        let elapsed_ms = (elapsed.as_secs() * 1000) + u64::from(elapsed.subsec_millis());
        let elapsed_sec = (elapsed_ms as f32) / 1000.0;
        data.i_global_time = elapsed_sec;
        data.i_time = elapsed_sec;

        // Resolution.
        data.i_resolution = [width, height, width / height];

        // Frame.
        data.i_frame += 1;

        // Draw.
        encoder.clear(&data.frag_color, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    });
}
