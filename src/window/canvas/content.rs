use cgmath::Deg;
use winit::event::{DeviceEvent,ElementState,KeyboardInput};

pub trait Content {
    fn new() -> Self;
    //fn layout_bindings() -> &[wgpu::BindingType];
    //fn bindings() -> &[wgpu::BindingResource];
    fn update(&mut self, event: &winit::event::DeviceEvent, device: &wgpu::Device) -> Option<wgpu::CommandBuffer>;
    fn update_camera(&mut self, event: &winit::event::DeviceEvent, camera: &mut crate::Camera) -> bool;
}

pub struct SimpleCamera {}

impl Content for SimpleCamera {
    fn new() -> SimpleCamera {
        SimpleCamera {}
    }

    fn update(&mut self, _event: &winit::event::DeviceEvent, _device: &wgpu::Device) -> Option<wgpu::CommandBuffer> {
        None
    }

    fn update_camera(&mut self, event: &winit::event::DeviceEvent, camera: &mut crate::Camera) -> bool {
        println!("scancode {:?}", &event);
        match event {
            DeviceEvent::MouseMotion { delta, ..} => {
                let (x,y): (f64,f64) = (*delta).into();
                camera.rotate(Deg(y as f32), Deg(x as f32));
                true
            },
            _ => false
        }
    }
}
