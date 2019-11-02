use crate::window::Window;
use crate::camera::Camera;

pub struct Canvas {
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    unif_camera: wgpu::Buffer,
    camera: Camera
}

impl Window for Canvas {
    fn new(sc_desc: &wgpu::SwapChainDescriptor, device: &wgpu::Device) -> Canvas {

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
        let ratio = sc_desc.width as f32/sc_desc.height as f32;
        let cam: [f32; 13] = [1.0, 0.0, 0.0,
                              0.0, 1.0, 0.0,
                              0.0, 0.0, 1.0,   

                              0.0, 0.0, 0.0,
                              ratio];
        let unif_camera = device.create_buffer_mapped(cam.len(), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,).fill_from_slice(&cam);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &unif_camera,
                        range: 0 .. 4*cam.len() as u64,
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

        Canvas {bind_group, pipeline, unif_camera, camera: Camera::new(ratio)}
    }

    fn update(&mut self, _event: winit::event::WindowEvent, device: &wgpu::Device) -> Option<wgpu::CommandBuffer> {
        let cam: [f32; 13] = [1.0, 0.0, 0.0,
                              0.0, 1.0, 0.0,
                              0.0, 0.0, 1.0,   

                              0.0, 0.0, 0.0,
                              self.camera.ratio];

        let temp_buf = device
            .create_buffer_mapped(cam.len(), wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&cam);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        encoder.copy_buffer_to_buffer(&temp_buf, 0, &self.unif_camera, 0, 4*cam.len() as u64);
        Some(encoder.finish())
    }

    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &wgpu::Device) -> Option<wgpu::CommandBuffer> {
        self.camera.ratio = sc_desc.width as f32/sc_desc.height as f32;
        let cam: [f32; 13] = [1.0, 0.0, 0.0,
                              0.0, 1.0, 0.0,
                              0.0, 0.0, 1.0,   

                              0.0, 0.0, 0.0,
                              self.camera.ratio];
        //TODO change ascpect ratio

        let temp_buf = device
            .create_buffer_mapped(cam.len(), wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&cam);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        encoder.copy_buffer_to_buffer(&temp_buf, 0, &self.unif_camera, 0, 4*cam.len() as u64);
        Some(encoder.finish())
    }

    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &wgpu::Device) -> wgpu::CommandBuffer {
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
