# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Make Glutin event loop and GL window visible in `lingo::window::Window` struct.

## [0.3.0] 2018-10-05
### Added
- Window builder helper `lingo::window::WindowBuilder`.
- Example code for stride.

### Changed
- Stride usage in hardware buffers.
- Rename attribute to pipeline.
- Simplified module paths for crate.

### Fixed
- Stride didn't work at all.

## [0.2.0] 2018-04-22
### Added
- Projections in `lingo::projection::Matrix4x4`.
- 2D Orthogonal projection.
- 3D perspective projection.
- Return standard Rust results in setup methods.
- More data types for attributes.
- Example code for camera.

## [0.1.0] - 2018-03-24
### Added
- GPU *Hardware* buffer abstraction `lingo::HwBuf`.
- Vertex buffer abstraction `lingo::Attribute`.
- Shader abstraction `lingo::shader::Program`.
- Window abstraction `lingo::window::Window`.
- Example code for *Hello triangle*.
