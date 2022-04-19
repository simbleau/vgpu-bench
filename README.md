# vgpu-bench
[![dependency status](https://deps.rs/repo/github/simbleau/vgpu-bench/status.svg)](https://deps.rs/repo/github/simbleau/vgpu-bench)
[![build](https://github.com/simbleau/vgpu-bench/workflows/build/badge.svg)](https://github.com/simbleau/vgpu-bench/actions/workflows/build.yml)

A benchmarking framework, specifically with a focus on hardware-accelerated graphical applications.

# Motivation & Research

This project focuses on benchmarking hardware-accelerated renderering approaches to export useful metrics and information for the field of 2D vector graphics. Moreover, this project can be used as a framework to provide metrics for use in a wider scope of research regarding hardware-accelerated applications. The subject matter has been documented in Spencer C. Imbleau's thesis work in partial fulfillment of a Master of Science degree in Computer Science.

# Quick Start
- `cargo run --example simple`

# Installation
## Requirements:
### Operating System
- Ubuntu 20.04 (other distros *may* work)
- Windows 10
### *(Optional) GPU Metric Sampling*
- **Graphics card**
  - NVIDIA Turing architecture or later
- **NVIDIA graphics card with minimum driver version:**
  - NVIDIA Turing architecture TU10x, TU11x - r440
  - NVIDIA Ampere architecture GA100 - r450
  - NVIDIA Ampere architecture GA100 MIG - r470 TRD1
  - NVIDIA Ampere architecture GA10x - r455
## Dependencies
- [Rust](https://www.rust-lang.org/tools/install)
- [NVIDIA NSight-Systems](https://developer.nvidia.com/nsight-systems)
- Ubuntu 20.04
  - `sudo apt install libgtk-3-dev`
- Windows
  - You'll have to figure this out for yourself right now...
## Install
 - Clone: `git clone git@github.com:simbleau/vgpu-bench.git`
 - Build: `cargo build` (`--release` to receive accurate benchmark data)

# Goal
The goal is to create an extensible and uniform benching framework which is capable of benchmarking hardware-accelerated applications with support for GPU metric sampling out of the box. Currently this is achieved by automatically augmenting GPU tracer annotations on benchmark function closures. The framework is designed to be cross-platform and able to run on recent GPUs, but currently only NVIDIA is supported.

# License
This project is dual-licensed under both [Apache 2.0](https://github.com/simbleau/convo/blob/main/LICENSE-APACHE) and [MIT](https://github.com/simbleau/convo/blob/main/LICENSE-MIT) licenses.
