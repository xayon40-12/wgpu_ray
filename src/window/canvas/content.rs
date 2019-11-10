use cgmath::Deg;
use winit::event::{Event,WindowEvent,DeviceEvent,ElementState,KeyboardInput};

pub trait Content {
    fn new() -> Self;
    fn layout_bindings(&self) -> &[wgpu::BindingType];
    fn bindings(&self) -> &[wgpu::BindingResource];
    fn update(&mut self, event: &Event<()>, device: &wgpu::Device) -> Vec<wgpu::CommandBuffer>;
    fn update_camera(&mut self, event: &Event<()>, camera: &mut crate::Camera) -> bool;
}

pub struct SimpleCamera {}

impl Content for SimpleCamera {
    fn new() -> SimpleCamera {
        SimpleCamera {}
    }

    fn layout_bindings(&self) -> &[wgpu::BindingType] {
        &[]
    }

    fn bindings(&self) -> &[wgpu::BindingResource] {
        &[]
    }

    fn update(&mut self, _event: &Event<()>, _device: &wgpu::Device) -> Vec<wgpu::CommandBuffer> {
        Vec::new()
    }

    fn update_camera(&mut self, event: &Event<()>, camera: &mut crate::Camera) -> bool {
        match event {
            Event::DeviceEvent { event, .. } => {
                match event {
                    DeviceEvent::MouseMotion { delta, ..} => {
                        let (x,y): (f64,f64) = (*delta).into();
                        camera.rotate(Deg(y as f32), Deg(x as f32));
                        true
                    },
                    _ => false
                }
            },
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { input: KeyboardInput { scancode, state: ElementState::Pressed, .. }, .. } => {
                        println!("{}", scancode);
                        match scancode {
                            13 => { camera.translate(1.0, 0.0, 0.0); true },
                            1 => { camera.translate(-1.0, 0.0, 0.0); true },
                            0 => { camera.translate(0.0, 1.0, 0.0); true },
                            2 => { camera.translate(0.0, -1.0, 0.0); true },
                            49 => { camera.translate(0.0, 0.0, 1.0); true },
                            56 => { camera.translate(0.0, 0.0, -1.0); true },
                            _ => false
                        }
                    },
                    _ => false
                }

            },
            _ => false
        }
    }
}
