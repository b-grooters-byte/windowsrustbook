# Hello, Direct2D

We are going to develop a simple static view for our first Direct2D application. This application is similar to the [Quick Start](https://learn.microsoft.com/en-us/windows/win32/direct2d/getting-started-with-direct2d) application in the Microsoft Windows Getting Started with Direct2D tutorial.

## The Project

Lets start by creating new Rust project:

```
cargo new hello_d2d
cd hello_d2d
```

The windows-rs crate makes extensive use of features. You should check out the [Cargo Book](https://doc.rust-lang.org/cargo/) section on [features](https://doc.rust-lang.org/cargo/reference/features.html) if you are unfamiliar with how features are used in Rust. 

We need to add the minimum set of window crate features to our ```Cargo.toml``` file that we need for our simple *Hello, Direct2D!* application. We use version 0.42 of the windows crate so lets define that: 

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


Next, we will move on  to the basic WIN32 application code.