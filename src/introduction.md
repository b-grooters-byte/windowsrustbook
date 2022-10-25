# Windows Direct2D Development with Rust

## Introduction

Welcome to *Windows Direct2D development with Rust*, an introductory book about developing [Direct2D](https://learn.microsoft.com/en-us/windows/win32/direct2d/direct2d-portal) applications with Windows and the [window-rs](https://github.com/microsoft/windows-rs) crate. 

Rust has many advantages over other languages available for Windows development and is particularly well suited to developing performant, safe, and reliable Direct2D applications. The ownership model in Rust helps ensures reliable memory and thread safe application development without sacrificing performance and readability of code. 

The [window-rs](https://github.com/microsoft/windows-rs) crate makes Windows APIs available to Rust developers. With the windows-rs you can now develop fully featured Windows applications that take advantage of the capabilities provided by Direct2D.

## Who should read this book?

This book is for anyone that wants to start developing Direct2D applications with the window-rs crate and Direct2D. It does not require extensive prior experience with Win32 development or Rust although an understanding of Win32 development will help you understand the concepts presented. 

This book assumes that you have a basic understanding of Rust. If this is not the case, we recommend [The Rust Programming Language](https://doc.rust-lang.org/stable/book/). For a more in depth understanding of Rust, we recommend [Rust for Rustaceans](https://rust-for-rustaceans.com/) 

## How this book is organized
The book is in 2 parts. 

* Part I is an introduction to various aspects of Direct2D and Win32 API; basic drawing, DirectWrite, and animation.

* Part 2 combines all of the concepts in part 1 into a fully functioning Direct2D implementation of the Windows classic Minesweeper game.

## Source Code

All the source code examples used in this book are available on [GitHub](https://github.com/bytetrail/windowsrustbook) along with the source to the book.

VS Code was used to develop the examples with the [rust-analyzer](https://code.visualstudio.com/docs/languages/rust) plugin. We do not decribe the [use](https://doc.rust-lang.org/std/keyword.use.html) imports in the book and in almost all cases the rust-analyzer plugin can be used to resolve the required imports. There are some cases where we were unable to resolve an import with Rust analyzer such as the ```windows::core::Result``` import; however, these are infrequent and can typically be looked up in the ```windows-rs``` crate doccumentation.

Along with rust-analyzer we like the [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) plugin for editing ```Cargo.toml``` files.

## License
The book is covered under the [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/) license.

The source code examples and code snippets included in the book from the examples are covered under the [MIT License](https://github.com/bytetrail/windowsrustbook/tree/main/examples)