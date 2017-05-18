use gfx;
use glutin;
use gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

use std::time::Instant;
use std::fs::File;
use std::io::Read;
use std::path::Path;

type ColorFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;

#[derive(PartialEq)]
enum Mouse {
    Released,
    Pressed,
}

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

pub fn run(w: f32, h: f32, shaderpath: &str) -> Result<(), String> {
    let (mut w, mut h) = (w, h);

    // Read fragment shader from file into byte buffer
    let mut frag_src_buf = Vec::new();
    match File::open(&Path::new(&shaderpath)) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_end(&mut frag_src_buf) {
                return Err(format!("Error reading from {}: {}", shaderpath, e));
            }
        },
        Err(e) => {
            return Err(format!("Error opening file {}: {}", shaderpath, e));
        }
    }
    let frag_src_buf = frag_src_buf.as_slice();

    // Read default vertex shader from file into byte buffer
    let vert_src_buf = include_bytes!("../shaders/default.vert");

    let builder = glutin::WindowBuilder::new()
        .with_title("shadertoy-rs")
        .with_dimensions(w as u32, h as u32)
        .with_vsync();

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();

    let pipeline;
    match factory.create_pipeline_simple(vert_src_buf, frag_src_buf, pipe::new()) {
        Ok(p)  => pipeline = p,
        Err(e) => return Err(format!("Error creating pipeline: {}", e)),
    }

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SCREEN, &SCREEN_INDICES[..]);

    let mut data = pipe::Data {
        vbuf: vertex_buffer,

        i_global_time: 0.0,
        i_resolution: [w, h, w/h],
        i_mouse: [0.0; 4],
        i_frame: -1,

        frag_color: main_color,
    };

    let mut last_mouse = Mouse::Released;
    let mut current_mouse = Mouse::Released;

    let (mut mx, mut my) = (0.0, 0.0);

    let mut xyzw = [0.0; 4];

    let start_time = Instant::now();

    loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                    glutin::Event::Closed => {
                        return Ok(());
                    },

                    glutin::Event::Resized(new_w, new_h) => {
                        gfx_window_glutin::update_views(&window, &mut data.frag_color, &mut main_depth);

                        w = new_w as f32;
                        h = new_h as f32;
                    }

                glutin::Event::MouseMoved(x, y) => {
                    mx = x as f32;
                    my = h - y as f32; // Flip y-axis
                },

                glutin::Event::MouseInput(state, button) => {
                    last_mouse = current_mouse;
                    if state == glutin::ElementState::Pressed && button == glutin::MouseButton::Left {
                        current_mouse = Mouse::Pressed;
                    } else {
                        current_mouse = Mouse::Released;
                    }
                }

                _ => ()
            }
        }

        // Mouse
        if current_mouse == Mouse::Pressed {
            xyzw[0] = mx;
            xyzw[1] = my;
            if last_mouse == Mouse::Released {
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
        data.i_resolution = [w, h, w/h];

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
