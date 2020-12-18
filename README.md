# rhythmc-rs

![Build & Check](https://github.com/patrickhadlaw/rhythmc-rs/workflows/Build%20&%20Check/badge.svg)

A Rust to GLSL/SPIR-V compiler framework. The goal of this crate is to allow
creation of GLSL shaders written is pure rust within a crate. This is useful
for the following reasons:
* Avoiding the annoyance of having to compile shaders manually (the resulting GLSL code and/or SPIR-V binary is written directly into the source code before the rust compiler gobbles it up).
* It provides a standardized way to import shader code. The way this will work is when you import a symbol using `use`, rhythmc will detect this and directly import the needed symbols into your GLSL code. So it's basically as simple as including from any other rust module.
* It allows you to write CPU simulated unit tests for your shader as if it was just rust code. That is, the rhythmc standard library will be implemented in the rust code and translated to GLSL when needed. So using the shader code in your rust project will 100% work (although it won't run on GPU).
* Eventually it will support an easy to use black box GPU based testing framework so you can test the resulting shader output.

Why it is called rhythmc?

The name roots from the Game Engine this framework is built for (a private project for now, maybe).

## Prerequisites

* Rust lang

## Authors

* **Patrick Hadlaw** - [patrickhadlaw](https://github.com/patrickhadlaw)

## Build instructions

```
$ cargo build
```

## Test instructions

```
$ cargo test
$ cd macros && cargo test
$ cd compiler && cargo test
```

