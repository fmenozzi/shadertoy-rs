#[macro_use]
extern crate clap;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use clap::App;

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

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut w: f32 = matches.value_of("width").unwrap().parse().expect("Invalid width specified");
    let mut h: f32 = matches.value_of("height").unwrap().parse().expect("Invalid height specified");

    // Read fragment shader from file into byte buffer
    let shaderpath = matches.value_of("shader").unwrap();
    let mut shader_src_buf = Vec::new();
    match File::open(&Path::new(shaderpath)) {
        Ok(mut file) => {
            match file.read_to_end(&mut shader_src_buf) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error reading from {}: {}", shaderpath, e);
                    return;
                }
            }
        },
        Err(e) => {
            println!("Error opening file {}: {}", shaderpath, e);
            return;
        }
    }

    let builder = glutin::WindowBuilder::new()
                    .with_title("Hello, gfx-rs!")
                    .with_dimensions(w as u32, h as u32)
                    .with_vsync();

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/default.vert"),
        shader_src_buf.as_slice(),
        pipe::new()
    ).expect("Error creating pipeline");

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
                    return;
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
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
