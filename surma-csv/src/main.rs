// use wgpu;
// use wgpu::util::DeviceExt;

async fn run() {
    let steps = execute_gpu().await.unwrap();
}


async fn execute_gpu() -> Option<Vec<u32>> {
    // Instantiates instance of WebGPU
    let instance = wgpu::Instance::new(wgpu::Backends::all());

    // `request_adapter` instantiates the general connection to the GPU
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;

    // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
    //  `features` being the available features.
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();

    let info = adapter.get_info();
    // skip this on LavaPipe temporarily
    if info.vendor == 0x10005 {
        return None;
    }

    println!("*** info: {:?}", info);

    return None;
}

fn main() {
    println!("*** surma-csv started...");

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
    // #[cfg(target_arch = "wasm32")]
    // {
    //     std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //     console_log::init().expect("could not initialize logger");
    //     wasm_bindgen_futures::spawn_local(run());
    // }
}
