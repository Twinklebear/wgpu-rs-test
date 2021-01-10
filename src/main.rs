use std::borrow::Cow;
use wgpu;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

async fn run(event_loop: EventLoop<()>, window: Window) {
    let window_size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find a WebGPU adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                shader_validation: false,
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Embedded SPV bytecode for our shaders
    let triangle_vert_spv = [
        0x07230203, 0x00010000, 0x000d0008, 0x00000018, 0x00000000, 0x00020011, 0x00000001,
        0x0006000b, 0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e, 0x00000000, 0x0003000e,
        0x00000000, 0x00000001, 0x0009000f, 0x00000000, 0x00000004, 0x6e69616d, 0x00000000,
        0x00000009, 0x0000000b, 0x00000012, 0x00000015, 0x00030003, 0x00000002, 0x000001c2,
        0x000a0004, 0x475f4c47, 0x4c474f4f, 0x70635f45, 0x74735f70, 0x5f656c79, 0x656e696c,
        0x7269645f, 0x69746365, 0x00006576, 0x00080004, 0x475f4c47, 0x4c474f4f, 0x6e695f45,
        0x64756c63, 0x69645f65, 0x74636572, 0x00657669, 0x00040005, 0x00000004, 0x6e69616d,
        0x00000000, 0x00040005, 0x00000009, 0x6c6f6366, 0x0000726f, 0x00040005, 0x0000000b,
        0x6c6f6376, 0x0000726f, 0x00060005, 0x00000010, 0x505f6c67, 0x65567265, 0x78657472,
        0x00000000, 0x00060006, 0x00000010, 0x00000000, 0x505f6c67, 0x7469736f, 0x006e6f69,
        0x00070006, 0x00000010, 0x00000001, 0x505f6c67, 0x746e696f, 0x657a6953, 0x00000000,
        0x00070006, 0x00000010, 0x00000002, 0x435f6c67, 0x4470696c, 0x61747369, 0x0065636e,
        0x00070006, 0x00000010, 0x00000003, 0x435f6c67, 0x446c6c75, 0x61747369, 0x0065636e,
        0x00030005, 0x00000012, 0x00000000, 0x00030005, 0x00000015, 0x00736f70, 0x00040047,
        0x00000009, 0x0000001e, 0x00000000, 0x00040047, 0x0000000b, 0x0000001e, 0x00000001,
        0x00050048, 0x00000010, 0x00000000, 0x0000000b, 0x00000000, 0x00050048, 0x00000010,
        0x00000001, 0x0000000b, 0x00000001, 0x00050048, 0x00000010, 0x00000002, 0x0000000b,
        0x00000003, 0x00050048, 0x00000010, 0x00000003, 0x0000000b, 0x00000004, 0x00030047,
        0x00000010, 0x00000002, 0x00040047, 0x00000015, 0x0000001e, 0x00000000, 0x00020013,
        0x00000002, 0x00030021, 0x00000003, 0x00000002, 0x00030016, 0x00000006, 0x00000020,
        0x00040017, 0x00000007, 0x00000006, 0x00000004, 0x00040020, 0x00000008, 0x00000003,
        0x00000007, 0x0004003b, 0x00000008, 0x00000009, 0x00000003, 0x00040020, 0x0000000a,
        0x00000001, 0x00000007, 0x0004003b, 0x0000000a, 0x0000000b, 0x00000001, 0x00040015,
        0x0000000d, 0x00000020, 0x00000000, 0x0004002b, 0x0000000d, 0x0000000e, 0x00000001,
        0x0004001c, 0x0000000f, 0x00000006, 0x0000000e, 0x0006001e, 0x00000010, 0x00000007,
        0x00000006, 0x0000000f, 0x0000000f, 0x00040020, 0x00000011, 0x00000003, 0x00000010,
        0x0004003b, 0x00000011, 0x00000012, 0x00000003, 0x00040015, 0x00000013, 0x00000020,
        0x00000001, 0x0004002b, 0x00000013, 0x00000014, 0x00000000, 0x0004003b, 0x0000000a,
        0x00000015, 0x00000001, 0x00050036, 0x00000002, 0x00000004, 0x00000000, 0x00000003,
        0x000200f8, 0x00000005, 0x0004003d, 0x00000007, 0x0000000c, 0x0000000b, 0x0003003e,
        0x00000009, 0x0000000c, 0x0004003d, 0x00000007, 0x00000016, 0x00000015, 0x00050041,
        0x00000008, 0x00000017, 0x00000012, 0x00000014, 0x0003003e, 0x00000017, 0x00000016,
        0x000100fd, 0x00010038,
    ];
    let triangle_frag_spv = [
        0x07230203, 0x00010000, 0x000d0008, 0x0000000d, 0x00000000, 0x00020011, 0x00000001,
        0x0006000b, 0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e, 0x00000000, 0x0003000e,
        0x00000000, 0x00000001, 0x0007000f, 0x00000004, 0x00000004, 0x6e69616d, 0x00000000,
        0x00000009, 0x0000000b, 0x00030010, 0x00000004, 0x00000007, 0x00030003, 0x00000002,
        0x000001c2, 0x000a0004, 0x475f4c47, 0x4c474f4f, 0x70635f45, 0x74735f70, 0x5f656c79,
        0x656e696c, 0x7269645f, 0x69746365, 0x00006576, 0x00080004, 0x475f4c47, 0x4c474f4f,
        0x6e695f45, 0x64756c63, 0x69645f65, 0x74636572, 0x00657669, 0x00040005, 0x00000004,
        0x6e69616d, 0x00000000, 0x00040005, 0x00000009, 0x6f6c6f63, 0x00000072, 0x00040005,
        0x0000000b, 0x6c6f6366, 0x0000726f, 0x00040047, 0x00000009, 0x0000001e, 0x00000000,
        0x00040047, 0x0000000b, 0x0000001e, 0x00000000, 0x00020013, 0x00000002, 0x00030021,
        0x00000003, 0x00000002, 0x00030016, 0x00000006, 0x00000020, 0x00040017, 0x00000007,
        0x00000006, 0x00000004, 0x00040020, 0x00000008, 0x00000003, 0x00000007, 0x0004003b,
        0x00000008, 0x00000009, 0x00000003, 0x00040020, 0x0000000a, 0x00000001, 0x00000007,
        0x0004003b, 0x0000000a, 0x0000000b, 0x00000001, 0x00050036, 0x00000002, 0x00000004,
        0x00000000, 0x00000003, 0x000200f8, 0x00000005, 0x0004003d, 0x00000007, 0x0000000c,
        0x0000000b, 0x0003003e, 0x00000009, 0x0000000c, 0x000100fd, 0x00010038,
    ];

    let vertex_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(
        Cow::Borrowed(&triangle_vert_spv),
    ));
    let fragment_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(
        Cow::Borrowed(&triangle_frag_spv),
    ));

    let vertex_data: [f32; 24] = [
        1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, -1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
    ];
    let data_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (vertex_data.len() * 4) as u64,
        usage: wgpu::BufferUsage::VERTEX,
        mapped_at_creation: true,
    });
    {
        // TODO: get_mapped_range_mut API not in the wgpu-rs web backend yet
        let mut view = data_buffer.slice(..).get_mapped_range_mut();
        let float_view = unsafe {
            std::slice::from_raw_parts_mut(view.as_mut_ptr() as *mut f32, vertex_data.len())
        };
        float_view.copy_from_slice(&vertex_data)
    }
    data_buffer.unmap();

    let vertex_attrib_descs = [
        wgpu::VertexAttributeDescriptor {
            offset: 0,
            format: wgpu::VertexFormat::Float4,
            shader_location: 0,
        },
        wgpu::VertexAttributeDescriptor {
            offset: 4 * 4,
            format: wgpu::VertexFormat::Float4,
            shader_location: 1,
        },
    ];

    let vertex_buffer_descs = [wgpu::VertexBufferDescriptor {
        stride: 2 * 4 * 4,
        step_mode: wgpu::InputStepMode::Vertex,
        attributes: &vertex_attrib_descs,
    }];

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swap_chain_format = wgpu::TextureFormat::Bgra8Unorm;
    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: swap_chain_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        },
    );

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vertex_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fragment_module,
            entry_point: "main",
        }),
        rasterization_state: None,
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: swap_chain_format,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            // Note: can use None in later relase of wgpu, since we don't use an index buffer
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &vertex_buffer_descs,
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let clear_color = wgpu::Color {
        r: 0.3,
        g: 0.3,
        b: 0.3,
        a: 1.0,
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. }
                    if input.virtual_keycode == Some(VirtualKeyCode::Escape) =>
                {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to get swapchain frame")
                    .output;
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(clear_color),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_vertex_buffer(0, data_buffer.slice(..));
                    render_pass.draw(0..3, 0..1);
                }
                queue.submit(Some(encoder.finish()));
            }
            _ => (),
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    {
        futures::executor::block_on(run(event_loop, window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("Failed to initialize logger");
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("Failed to append canvas to body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}
