use windows::{
    core::Result,
    Win32::Graphics::Direct2D::{
        D2D1CreateFactory, ID2D1Factory1, D2D1_DEBUG_LEVEL_INFORMATION, D2D1_FACTORY_OPTIONS,
        D2D1_FACTORY_TYPE_SINGLE_THREADED,
    },
};

/// Creates a single threaded Direct2D factory with default options.
pub fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&options)) }
}
