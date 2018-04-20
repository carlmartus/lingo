# Lingo
[![Build Status](https://travis-ci.org/carlmartus/lingo.svg?branch=master)](https://travis-ci.org/carlmartus/lingo)

Rust OpenGL ES 2.0 library.

Using the OpenGL ES 2.0 API directly can be a lot of work.
This library is meant to make this job easier.

## Features
 * `lingo::window`, OpenGL window setup with input handling.
 * `lingo::shader`, shader loading.
 * `lingo::hwbuf`, vertex array buffer handling.
 * `lingo::attribute`, render setup by defining attributes.
 * `lingo::error`, OpenGL error checking.
 * `lingo::projection`, create projection matrices.

## Sample
 * [Hello triangle](samples/hello_triangle.rs) - *Hello world* but for GPU.
 * [3D camera](samples/camera.rs) - Using projections to get a 3D camera
   perspective.
