use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustwine_dxvk::d3d11::D3D11Device;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Direct3D::*;

fn graphics_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("graphics");
    
    // Benchmark device creation
    group.bench_function("create_device", |b| {
        b.iter(|| {
            D3D11Device::new().unwrap();
        });
    });
    
    // Benchmark swapchain creation
    group.bench_function("create_swapchain", |b| {
        let device = D3D11Device::new().unwrap();
        let mut desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width: 1920,
                Height: 1080,
                RefreshRate: DXGI_RATIONAL {
                    Numerator: 60,
                    Denominator: 1,
                },
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
            },
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            OutputWindow: std::ptr::null_mut(),
            Windowed: true.into(),
            SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
            Flags: 0,
        };
        
        b.iter(|| {
            device.create_swapchain(black_box(&desc)).unwrap();
        });
    });
    
    // Benchmark present
    group.bench_function("present", |b| {
        let device = D3D11Device::new().unwrap();
        
        b.iter(|| {
            device.present().unwrap();
        });
    });
    
    group.finish();
}

criterion_group!(benches, graphics_benchmark);
criterion_main!(benches); 