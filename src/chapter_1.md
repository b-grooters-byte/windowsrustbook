# Getting Started

## Setup

You need to have the Rust toolchain installed on your machine. The easiest way to do this if you have not already done it is to install the toolchain via [rustup](https://rustup.rs/).

All of the code examples in this book have been developed with Rust 1.64.0. Earlier versions may work as well; however, we recommend you update to the latest stable version of Rust. Use rustup to check your version of Rust.

```
rustup --version
```

Update with rustup to Rust 1.64.0 or later (as of 23-OCT-2022 this is 1.66.0):

```
rustup update stable
```

## Windows and Direct2D

The Windows 11 build used for the code samples is ```Version 10.0.22621 Build 22621``` with Direct2D. You will not need Visual Studio or the Microsoft Windows SDK unless you plan on building the windows-rs crate yourself. This is covered in more detail in [Appendix A - Building the windows-rs crate](./appendix_a.md).

