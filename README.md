# vgpu-bench
[![dependency status](https://deps.rs/repo/github/simbleau/vgpu-bench/status.svg)](https://deps.rs/repo/github/simbleau/vgpu-bench)
[![Build Status](https://travis-ci.com/simbleau/vgpu-bench.svg?branch=main)](https://travis-ci.com/simbleau/vgpu-bench) 
[![GitHub license](https://img.shields.io/github/license/simbleau/vgpu-bench)](https://github.com/simbleau/vgpu-bench/blob/main/LICENSE)

A benching framework for third party vector graphics libraries, techniques, and renderers.

# Motivation

This project focuses on benching existing (usually hardware-accelerated) vector graphic utilities to export useful metrics and information. The information would accentuate provable theories about vector graphics as well as discover and highlight areas of friction and avant achievement.

This project is used as a framework to provide metrics for use in a wider scope of research regarding hardware-accelerated vector graphics.

# Sections

* [Motivation](#motivation)
* [Methodology](#methodology)
* [State](#state)
* [FAQ](#faq)

## Methodology

The goal is to create a uniform benching framework which is capable of operating on a vector graphic test-suite. The framework is designed to be cross-platform and able to run on various hardware and GPU architecture (particularly NVIDIA).
Individual tests should be accentuated with GPU annotations and events (via [nvtx-rs](https://github.com/simbleau/nvtx-rs)) for fine-tuned metrics.

## State

vgpu-bench is in active research and development. Currently this is in research phase as all code is still in flux. 

The code here does not intend to be released as an abstract framework, but should be extensible enough to add a new unit for comparison. \
No releases will be published, but findings and metrics from this project should be publicly available when it's ready.

## FAQ

### In a nutshell, what are vector graphics?

Vector graphics directly oppose raster graphics by storing data in a different, implicit form which inherently mainly provides smaller file size, lossless quality, and abstract scaling.

### Why vector graphics?

Vector graphics are the ideal representation for visualizing abstract sizes. Optimization of vector graphics is a widely unsolved issue, so the metrics presented by this project should be leveraged by new, experimental renderers to improve their algorithms.

### In a nutshell, what is a renderer?

Renderers provide a graphic API abstraction to the user wherein users make calls to draw elements on a frame buffer in a window.

## Contributing

For now, please make direct contact with the author if you wish to contribute. Right now this is largely an individual effort. \
Don't hesitate to [file an issue](https://github.com/simbleau/vgpu-bench/issues/new) or contact [@simbleau](https://github.com/simbleau) by [e-mail](mailto:spencer@imbleau.com).
