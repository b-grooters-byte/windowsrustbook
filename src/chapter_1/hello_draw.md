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

The ```create_factory``` function will create a ```D2D1Factory1``` struct instance. ```D2D1Factory1``` implements a number of traits including ```From``` other Direct2D factory types. You will typically not need to convert the ```D2D1Factory1``` to other factory types but, would create the factory type required. We will stay with the ```D2D1Factory1``` for the projects we cover. ([Rust](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Direct2D/struct.ID2D1Factory1.html) | [WIN32](https://learn.microsoft.com/en-us/windows/win32/api/d2d1/nn-d2d1-id2d1factory)).

A single threaded factory type is created by this wrapper. A multi-threaded factory supports sharing of resources across threads but, that will not be required for any project in this book. 

The [```cfg!```](https://doc.rust-lang.org/std/macro.cfg.html) macro checks for whether or not debug assertions have been enabled as a build compile time flag. The cfg! macro evaluates to ```true``` or ```false``` and does not conditionally include code like the [```#[cfg]```](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute) attribute.

## Create the Direct2D factory

``` rust
fn main() -> Result<()> {
    let factory = direct2d::create_factory()?;
...
```

## Use the factory in MainWindow

``` rust
pub struct MainWindow<'a> {
    handle: HWND,
    factory: &'a ID2D1Factory1,
}
```