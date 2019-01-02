# Lingo
[![Build Status](https://travis-ci.org/carlmartus/lingo.svg?branch=master)](https://travis-ci.org/carlmartus/lingo)

Rust wrapper library for OpenGL.

Using the OpenGL API directly can be a lot of work.
This library is meant to make this job easier.

## Modules
 * `lingo::draw::HwBuf`, vertex array buffer handling.
 * `lingo::draw::Matrix4x4`, create projection matrices.
 * `lingo::draw::Pipeline`, render setup by defining a pipeline.
 * `lingo::draw::Program`, GLSL program abstraction.
 * `lingo::draw::ProgramBuilder`, makes GLSL shaders a bit easier.
 * `lingo::draw::print_gl_error`, OpenGL error checking.
 * `lingo::window`, OpenGL window setup with input handling.

## Sample
- [Hello triangle](samples/hello_triangle.rs), *Hello world* but for GPU.
- [Stride](samples/stride.rs), multiple vertex attributes using stride.
- [3D camera](samples/camera.rs), using projections to get a 3D camera
  perspective.
- [Events](samples/events.rs), prints all window events on `stdout`.

