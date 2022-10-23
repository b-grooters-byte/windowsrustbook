# Hello, Direct2D

We are going to develop a simple static view for our first Direct2D application. This application is similar to the [Quick Start](https://learn.microsoft.com/en-us/windows/win32/direct2d/getting-started-with-direct2d) application in the Microsoft Windows Getting Started with Direct2D tutorial.

Lets start by creating new Rust project:

```cargo new hello_d2d```

The windows-rs crate makes extensive use of features. You should check out the [Cargo Book](https://doc.rust-lang.org/cargo/) section on [features](https://doc.rust-lang.org/cargo/reference/features.html) if you are unfamiliar with how features are used in Rust. 

We need to start by adding the minimum set of window crate features to our ```Cargo.toml``` file that we need for our simple *Hello, World* application. We use version 0.42 of the windows crate so lets start there: 

``` toml
#[dependencies.windows]
version = "0.42"
```

We will use some basic foundational windows crate features for every Win32 application in this book so we will need to add those to the cargo file. Under the ```version = 0.42"``` that you just added, add the following features:

``` toml
features = [
    "Win32_Foundation",
    "Win32_Graphics_Direct2D",
    "Win32_Graphics_Gdi",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_LibraryLoader"
]

```

Under the ```src``` path open the ```main.rs``` file. We are going to start with a minimal Win32 application and explain the basics before we add the Direct2D rendering.

We need to create a struct that will represent our window

``` rust
pub struct MainWindow {
        handle: HWND,
}
```
We are only going to store the HWND for now so this is the only field in the structure. It will be a private field since no functions outside the ```MainWindow``` implementation will need to access it.

* NOTE: [HWND](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-windows#window-handle) is a unique identifier of a window instance in Win32.

The implementation of the ```MainWindow``` will create and display a simple [OVERLAPPED](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles) window. We need to create a ```new``` method and a windows procedure (wnd_proc) for a minimal Win32 Application.

* NOTE: We have to use some ```unsafe``` code with the windows crate and we will keep the unsafe scopes to a minimum. ```unsafe``` can be a deep topic in Rust and for the purposes of this book we will use unsafe code from the windows crate but we will not be using any unsafe Rust features outside of that context.

We will need a couple of constants to register our new window class so lets go ahead and create those just after the using statements:

``` rust
const WINDOW_CLASSNAME: &HSTRING = w!("bytetrail.rustd2d.hello");
const WINDOW_TITLE: &HSTRING = w!("Hello!");
```

These constants are defined as references to ```HSTRING```s. The ```w!``` macro is used to convert a ```'static &str``` to a ```&HSTRING```. We use HSTRING constants here because they will be needed when we register our windows class and give it a window title.

We define a static instance of a Once synchronization primitive at the top of the file, typically just before or after your consts. This will be used when we register the window class below :

``` rust 
static REGISTER_WINDOW_CLASS: Once = Once::new();
```

Lets start with the new method for our MainWindow. The signature of the new method may look a little strange if you have not used [```Box```](https://doc.rust-lang.org/std/boxed/struct.Box.html) before. We are using a ```Box```ed return type because we will need a heap allocated instance of the MainWindow later. 

The first thing we need do is get an instance handle that we will use when we create the window with the ```CreateWindowExW``` Win32 function:

``` rust
    pub fn new() -> Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None)? };
```

Next we register the windows class. This is done with a [```Once```](https://doc.rust-lang.org/std/sync/struct.Once.html) synchronization primative so that the windows class is registered only one time in the application no matter how many times new is called. Within the closure of ```call_once``` we define a windows class and register it

```rust 
        // synchronization for a one time initialization of FFI call
        REGISTER_WINDOW_CLASS.call_once(|| {
            // use defaults for all other fields
            let class = WNDCLASSW {
                lpfnWndProc: Some(Self::wnd_proc),
                hbrBackground: HBRUSH(COLOR_WINDOW.0 as isize),
                hInstance: instance,
                style: CS_HREDRAW | CS_VREDRAW,
                hCursor: unsafe { LoadCursorW(HINSTANCE(0), IDC_ARROW).ok().unwrap() },
                lpszClassName: WINDOW_CLASSNAME.into(),
                ..Default::default()
            };
            assert_ne!(unsafe { RegisterClassW(&class) }, 0);
        });
```

If you are completely new to Win32 development there is a lot to unpack there. Inside the ```call_once()``` closure we are calling [```WNDCLASSW```](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/struct.WNDCLASSW.html) with the required parameters. The ```lpfnWndProc``` field points to a method in our ```MainWindow``` class that we have not written yet. 

Next we set the background brush. The Win32 API defines a number of standard colors that are available through the windows crate although we have to do some casting to get them to the type we want. 

We set the ```hInstance``` to the instance we got earlier and the ```style```, and ```hcursor``` are typical for a top level window.

The ```lpszClassName``` uses the ```HSTRING``` constant we defined at the top of the file to uniquely identify this windows class.

The next line :
``` rust 
    ..Default::default()
```

sets the remainder of the fields to default values for the ```WNDCLASSW``` struct. The [```std::default::Default```](https://dev-doc.rust-lang.org/beta/std/default/trait.Default.html) trait documentation describes this usage. 

The next line registers the windows class using the [```RegisterClassW```](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.RegisterClassW.html) function with the ```WNDCLASSW``` struct instance and asserts that the registration was successful by comparing the return to 0.

You can learn more about defining and registering windows classes with the [Microsoft API documentation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)

``` rust
        let mut main_window = Box::new(MainWindow {
            handle: HWND(0),
            target: None,
        });
```

``` rust
        // create the window using Self reference
        let window = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                WINDOW_CLASSNAME,
                WINDOW_TITLE,
                WS_VISIBLE | WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                400,
                300,
                HWND(0),
                HMENU(0),
                instance,
                Some(main_window.as_mut() as *mut _ as _),
            )
        };
        unsafe { ShowWindow(window, SW_SHOW) };
        Ok(main_window)
    }
```

