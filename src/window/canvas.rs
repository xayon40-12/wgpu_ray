use crate::Window;
use crate::Camera;

use winit::event::Event;
use cgmath::Deg;

pub mod content;
use content::Content;

pub struct Canvas<T: 'static + Content> {
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    unif_camera: wgpu::Buffer,
    camera: Camera,
    content: T,
}

impl<T: 'static + Content> Window for Canvas<T> {
    fn new(sc_desc: &wgpu::SwapChainDescriptor, device: &wgpu::Device) -> Canvas<T> {
        let content = T::new();

        let vs = include_str!("shader.vert"); 
        let fs = include_str!("shader.frag");

        let vs_module = device.create_shader_module(&wgpu::read_spirv(glsl_to_spirv::compile(&vs, glsl_to_spirv::ShaderType::Vertex).unwrap()).unwrap());
        let fs_module = device.create_shader_module(&wgpu::read_spirv(glsl_to_spirv::compile(&fs, glsl_to_spirv::ShaderType::Fragment).unwrap()).unwrap());

        let mut layout_bindings = Vec::new();
        layout_bindings.push(
            wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                },
            }
        );
        content.layout_bindings().iter().enumerate().for_each(|(i,e)| { layout_bindings.push(
            wgpu::BindGroupLayoutBinding {
                binding: 1+i as u32,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: e.clone()
            }
        )});

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &layout_bindings,
        });

        
        let ratio = sc_desc.width as f32/sc_desc.height as f32;
        let camera = Camera::new(ratio,Deg(30.0));
        let cam = camera.as_float_array();
        let unif_camera = device.create_buffer_mapped(cam.len(), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST).fill_from_slice(&cam);

        let mut bindings = Vec::new();
        bindings.push(
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &unif_camera,
                    range: 0 .. 4*cam.len() as u64,
                },
            }
        );
        content.bindings().iter().enumerate().for_each(|(i,e)| { bindings.push(
            wgpu::Binding {
                binding: 1+i as u32,
                resource: e.clone()
            }
        )});

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &bindings,
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

        Canvas {bind_group, pipeline, unif_camera, camera, content}
    }

    fn update(&mut self, event: Event<()>, device: &wgpu::Device) -> Vec<wgpu::CommandBuffer> {
        let mut command_bufs = Vec::new();
        if self.content.update_camera(&event, &mut self.camera) {
            command_bufs.push(self.update_cam(&device));
        }
        command_bufs.append(&mut self.content.update(&event, &device));

        command_bufs
    }

    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &wgpu::Device) -> Option<wgpu::CommandBuffer> {
        self.camera.set_ratio(sc_desc.width as f32/sc_desc.height as f32);
        Some(self.update_cam(&device))
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

impl<T: 'static + Content> Canvas<T> {
    fn update_cam(&mut self, device: &wgpu::Device) -> wgpu::CommandBuffer {
        let cam = self.camera.as_float_array();

        let temp_buf = device
            .create_buffer_mapped(cam.len(), wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&cam);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        encoder.copy_buffer_to_buffer(&temp_buf, 0, &self.unif_camera, 0, 4*cam.len() as u64);
        encoder.finish()
    }
}
