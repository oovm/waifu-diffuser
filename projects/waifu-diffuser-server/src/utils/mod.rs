use pyke_diffusers::{ArenaExtendStrategy, CUDADeviceOptions, CuDNNConvolutionAlgorithmSearch, DiffusionDevice};

pub fn cuda_device(index: usize) -> DiffusionDevice {
    DiffusionDevice::CUDA(
        index as i32,
        Some(CUDADeviceOptions {
            memory_limit: None,
            arena_extend_strategy: Some(ArenaExtendStrategy::SameAsRequested),
            cudnn_conv_algorithm_search: Some(CuDNNConvolutionAlgorithmSearch::Exhaustive),
        }),
    )
}
