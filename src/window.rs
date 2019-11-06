pub mod canvas;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{self,Event,WindowEvent,DeviceEvent},
};

pub trait Window {
    fn new(sc_desc: &wgpu::SwapChainDescriptor, device: &wgpu::Device) -> Self;
    fn update(&mut self, event: DeviceEvent, device: &wgpu::Device) -> Option<wgpu::CommandBuffer>;
    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &wgpu::Device) -> Option<wgpu::CommandBuffer>;
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &wgpu::Device) -> wgpu::CommandBuffer;
}

pub fn run<T: 'static + Window>() {
    let events_loop = EventLoop::new();

    let (window, size, surface) = {
        let window = winit::window::Window::new(&events_loop).unwrap();
        window.set_cursor_grab(true).expect("Could not grab cursor");
        window.set_cursor_visible(false);
        //window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(window.current_monitor())));
        let size = window.inner_size().to_physical(window.hidpi_factor());
        let surface = wgpu::Surface::create(&window);

        (window, size, surface)
    };

    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        backends: wgpu::BackendBit::PRIMARY
    }).unwrap();

    let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false
        },
        limits: wgpu::Limits::default()
    });

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width.round() as u32,
        height: size.height.round() as u32,
        present_mode: wgpu::PresentMode::Vsync,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);
    
    let mut win = T::new(&sc_desc, &device);

    events_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    let physical = size.to_physical(window.hidpi_factor());
                    sc_desc.width = physical.width.round() as u32;
                    sc_desc.height = physical.height.round() as u32;
                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                    let command_buf = win.resize(&sc_desc, &device);
                    if let Some(command_buf) = command_buf {
                        queue.submit(&[command_buf]);
                        window.request_redraw();
                    }
                },
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::RedrawRequested => {
                    let frame = swap_chain.get_next_texture();
                    let command_buf = win.render(&frame, &device);
                    queue.submit(&[command_buf]);
                },
                _ => {}
            },
            Event::DeviceEvent { event, .. } => {
                let command_buf = win.update(event, &device);
                if let Some(command_buf) = command_buf {
                    queue.submit(&[command_buf]);
                    window.request_redraw();
                }
            }
            _ => ()
        }
    });
}

