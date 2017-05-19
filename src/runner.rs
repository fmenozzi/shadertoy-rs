use argvalues::ArgValues;

use loader;

use gfx;
use glutin;
use gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use glutin::{VirtualKeyCode, ElementState, MouseButton, Event};

use std::time::Instant;

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
        i_resolution: gfx::Global<[f32; 3]> = "iResolution",
        i_mouse: gfx::Global<[f32; 4]> = "iMouse",
        i_frame: gfx::Global<i32> = "iFrame",
        i_channel0: gfx::TextureSampler<[f32; 4]> = "iChannel0",
        i_channel1: gfx::TextureSampler<[f32; 4]> = "iChannel1",
        i_channel2: gfx::TextureSampler<[f32; 4]> = "iChannel2",
        i_channel3: gfx::TextureSampler<[f32; 4]> = "iChannel3",

        // Output color
        frag_color: gfx::BlendTarget<ColorFormat> = ("fragColor", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
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

pub fn run(av: &ArgValues) -> Result<(), String> {
    let mut width = av.width;
    let mut height = av.height;

    let texture0path = &av.texture0path;
    let texture1path = &av.texture1path;
    let texture2path = &av.texture2path;
    let texture3path = &av.texture3path;

    // Load vertex and fragment shaders into byte buffers
    let (vert_src_res, frag_src_res) = loader::load_shaders(&av);
    let (vert_src_buf, frag_src_buf): (Vec<u8>, Vec<u8>);
    match (vert_src_res, frag_src_res) {
        (Ok(vsbuf), Ok(fsbuf)) => {
            vert_src_buf = vsbuf;
            frag_src_buf = fsbuf;
        },

        (Err(vse), _) => return Err(format!("Error reading vertex shader: {}", vse)),
        (_, Err(fse)) => return Err(format!("Error reading fragment shader: {}", fse)),
    }
    let (vert_src_buf, frag_src_buf) = (vert_src_buf.as_slice(), frag_src_buf.as_slice());

    let builder = glutin::WindowBuilder::new()
        .with_title("shadertoy-rs")
        .with_dimensions(width as u32, height as u32)
        .with_vsync();

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();

    let mut pipeline;
    match factory.create_pipeline_simple(vert_src_buf, frag_src_buf, pipe::new()) {
        Ok(p)  => pipeline = p,
        Err(e) => return Err(format!("Error creating pipeline: {}", e)),
    }

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SCREEN, &SCREEN_INDICES[..]);

    // Load textures
    let sampler = factory.create_sampler_linear();
    let texture0;
    let texture1;
    let texture2;
    let texture3;
    match loader::load_texture(&texture0path, &mut factory) {
        Ok(tex) => texture0 = tex,
        Err(e)  => return Err(e),
    }
    match loader::load_texture(&texture1path, &mut factory) {
        Ok(tex) => texture1 = tex,
        Err(e)  => return Err(e),
    }
    match loader::load_texture(&texture2path, &mut factory) {
        Ok(tex) => texture2 = tex,
        Err(e)  => return Err(e),
    }
    match loader::load_texture(&texture3path, &mut factory) {
        Ok(tex) => texture3 = tex,
        Err(e)  => return Err(e),
    }

    let mut data = pipe::Data {
        vbuf: vertex_buffer,

        i_global_time: 0.0,
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

    loop {
        for event in window.poll_events() {
            match event {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) | Event::Closed => {
                    return Ok(());
                },

                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::F5)) => {
                    // Reload fragment shader into byte buffer
                    let frag_src_res = loader::load_fragment_shader(&av);
                    let frag_src_buf: Vec<u8>;
                    match frag_src_res {
                        Ok(fsbuf) => frag_src_buf = fsbuf,
                        Err(fse)  => return Err(format!("Error reloading fragment shader: {}", fse)),
                    }
                    let frag_src_buf = frag_src_buf.as_slice();

                    // Recreate pipeline
                    match factory.create_pipeline_simple(vert_src_buf, frag_src_buf, pipe::new()) {
                        Ok(p)  => pipeline = p,
                        Err(e) => return Err(format!("Error recreating pipeline: {}", e)),
                    }

                    // Reset uniforms
                    data.i_global_time = 0.0;
                    data.i_resolution = [width, height, width/height];
                    data.i_mouse = [0.0; 4];
                    data.i_frame = -1;

                    start_time = Instant::now();
                },

                Event::Resized(new_width, new_height) => {
                    gfx_window_glutin::update_views(&window, &mut data.frag_color, &mut main_depth);

                    width = new_width as f32;
                    height = new_height as f32;
                },

                Event::MouseMoved(x, y) => {
                    mx = x as f32;
                    my = height - y as f32; // Flip y-axis
                },

                Event::MouseInput(state, button) => {
                    last_mouse = current_mouse;
                    if state == ElementState::Pressed && button == MouseButton::Left {
                        current_mouse = ElementState::Pressed;
                    } else {
                        current_mouse = ElementState::Released;
                    }
                },

                _ => ()
            }
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
        let elapsed_ms = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos()/1000000) as u64;
        let elapsed_sec = (elapsed_ms as f32) / 1000.0;
        data.i_global_time = elapsed_sec;

        // Resolution
        data.i_resolution = [width, height, width/height];

        // Frame
        data.i_frame += 1;

        // Draw
        encoder.clear(&data.frag_color, CLEAR_COLOR);
        encoder.draw(&slice, &pipeline, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
