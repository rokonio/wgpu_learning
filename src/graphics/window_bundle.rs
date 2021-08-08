#![allow(unused)]
use winit::window::Window;

pub struct WindowBundle {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl WindowBundle {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        // An instance is a bridge beetween the app and the API
        // (here PRIMARY mean Vulkan + Metal + DX12 + Browser WebGPU)
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        // A surface is where the image will be rendered on (e.g the window)
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        // The device is the connection beetween the program and the GPU.
        // The queue is the gpu's equivalent of a thread pool (I think)
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // TODO maybe tyr adding a trace to see what's happening
            )
            .await
            .unwrap();

        // The swapchain is a buffer of two or three images. One is showed on
        // the screen while the others are computing. Whenever one is finished
        // it is swaped with the shown image.
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
        }
    }
}
