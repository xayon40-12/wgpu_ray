use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{self,WindowEvent},
};

pub struct Window {
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    unif_camera: wgpu::Buffer
}

impl Window {
    pub fn new(device: &wgpu::Device) -> Window {

        let vs = include_str!("shader.vert"); 
        let fs = include_str!("shader.frag");

        let vs_module = device.create_shader_module(&wgpu::read_spirv(glsl_to_spirv::compile(&vs, glsl_to_spirv::ShaderType::Vertex).unwrap()).unwrap());
        let fs_module = device.create_shader_module(&wgpu::read_spirv(glsl_to_spirv::compile(&fs, glsl_to_spirv::ShaderType::Fragment).unwrap()).unwrap());

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                    },
                },
            ],
        });

        
        //TODO add camera inside Window
        let cam: [f32; 12] = [1.0, 0.0, 0.0,
                              0.0, 1.0, 0.0,
                              0.0, 0.0, 1.0,   

                              0.0, 0.0, 0.0];
        let unif_camera = device.create_buffer_mapped(12, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,).fill_from_slice(&cam);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &unif_camera,
                        range: 0 .. 4*12,
                    },
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Window {bind_group, pipeline, unif_camera}
    }

    pub fn run() {

        let events_loop = EventLoop::new();

        let (_window, size, surface) = {
            let window = winit::window::Window::new(&events_loop).unwrap();
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

        let mut swap_chain = device.create_swap_chain(&surface, &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width.round() as u32,
            height: size.height.round() as u32,
            present_mode: wgpu::PresentMode::Vsync,
        });
        
        let mut win = Self::new(&device);

        events_loop.run(move |event, _, control_flow| {
            match event {
                event::Event::WindowEvent { event, .. } => match event {
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
                    }
                    _ => {
                        win.update(event, &device);
                    }
                },
                event::Event::EventsCleared => {
                    let frame = swap_chain.get_next_texture();
                    let command_buf = win.render(&device, &frame);
                    queue.submit(&[command_buf]);
                },
                _ => ()
            }
        });
    }

    fn update(&mut self, _event: winit::event::WindowEvent, _device: &wgpu::Device) {
        //empty
    }


    fn render(&mut self, device: &wgpu::Device, frame: &wgpu::SwapChainOutput) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw(0 .. 6, 0 .. 1);
        }

        encoder.finish()
    }
}


