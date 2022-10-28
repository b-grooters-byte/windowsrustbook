# Hello, Direct2D - Drawing

We need to start preparing our existing solution for Direct2D drawing. First we will update our ```Cargo.toml``` to include a Direct2D feature from the windows crate that we will need. Add the following to the ```features``` section of the ```[dependencies.windows]```:

``` toml
[dependencies.windows]
version = "0.42" 
features = [
 ...
 "Win32_Graphics_Direct2D",
]
 
```

## Direct2D Functions
The windows crate makes all of the WIN32 Direct2D functions available to us; however, our usage of the API is relatively narrow and follows some repeated patterns. We are going to create wrapper functions that encapsulate our usage of the Direct2D API where appropriate.

Create a file named ```direct2d.rs``` in the ```src``` folder and add the new module to your ```main.rs``` file. Add the following to the top of the ```main.rs``` file:

``` rust
mod direct2d;

```

In the ```direct2d.rs``` file we will add the first wrapper function that we will need. This will be a function to create a Direct2D factory. A Direct2D factory is used to create other Direct2D resources. The factory is typically created once at application start and shared with all objects that need it. 

``` rust
pub fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&options)) }
}

```

The ```create_factory``` function will create a ```D2D1Factory1``` ([Rust](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Direct2D/struct.ID2D1Factory1.html) | [WIN32](https://learn.microsoft.com/en-us/windows/win32/api/d2d1_1/nn-d2d1_1-id2d1factory1)) instances and return that wrapped in a ```windows::core::Result```. 

A single threaded factory type is created by this wrapper. A multi-threaded factory supports sharing of resources across threads but, that will not be required for any project in this book. 