# Lingo
[![Build Status](https://travis-ci.org/carlmartus/lingo.svg?branch=master)](https://travis-ci.org/carlmartus/lingo)

Rust OpenGL ES 2.0 library.

Using the OpenGL ES 2.0 API directly can be a lot of work.
This library is meant to make this job easier.

## Modules
 * `lingo::draw::HwBuf`, vertex array buffer handling.
 * `lingo::draw::Matrix4x4`, create projection matrices.
 * `lingo::draw::Pipeline`, render setup by defining a pipeline.
 * `lingo::draw::Program`, shader loading.
 * `lingo::draw::print_gl_error`, OpenGL error checking.
 * `lingo::window`, OpenGL window setup with input handling.

## Sample
- [Hello triangle](samples/hello_triangle.rs), *Hello world* but for GPU.
- [3D camera](samples/camera.rs), using projections to get a 3D camera
  perspective.
- [Stride](samples/stride.rs), multiple vertex attributes using stride.

