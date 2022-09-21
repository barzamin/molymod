use log::info;
use thiserror::Error;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

#[derive(Debug, Error)]
enum RendererError {
    #[error("no adapter found for surface/backend combo")]
    NoAdapter,

    #[error("error requesting device: {0}")]
    ReqDevError(#[from] wgpu::RequestDeviceError),

    #[error("surface error: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),
}

struct RenderState {
    surface: wgpu::Surface,
    surf_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
}

impl RenderState {
    pub async fn new(window: &Window) -> Result<Self, RendererError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                power_preference: wgpu::PowerPreference::default(),
            })
            .await
            .ok_or(RendererError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await?;

        let surf_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surf_config);

        Ok(Self {
            surface,
            surf_config,
            device,
            queue,
            size,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.size = new_size;
        self.surf_config.width = self.size.width;
        self.surf_config.height = self.size.height;
        self.surface.configure(&self.device, &self.surf_config);
    }

    pub fn render(&mut self) -> Result<(), RendererError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render encoder"),
        });
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("test pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        
        output.present();

        Ok(())
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ev_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&ev_loop)?;

    let mut state = RenderState::new(&window).await?;
    ev_loop.run(move |ev, _, ctl_flow| {
        ctl_flow.set_poll(); // continuously run the event loop

        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    info!("window close requested; shutting down");
                    ctl_flow.set_exit();
                }

                WindowEvent::Resized(physical_size) => state.resize(physical_size),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(*new_inner_size)
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                state.render().unwrap();
            }
            _ => (),
        }
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(run())
}
