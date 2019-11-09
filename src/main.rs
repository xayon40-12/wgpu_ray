use wgpu_ray::window;
use wgpu_ray::Canvas;
use wgpu_ray::SimpleCamera;

fn main() {
    window::run::<Canvas<SimpleCamera>>();
}
