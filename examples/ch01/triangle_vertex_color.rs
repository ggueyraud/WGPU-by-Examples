mod common;

use std::borrow::Cow;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    window.set_title("ch01_triangle_vertex_color");
    env_logger::init();

    let mut inputs = common::Inputs {
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("triangle-vertex-color.wgsl"))),
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
    };

    pollster::block_on(inputs.new(event_loop, window, 3));
}
