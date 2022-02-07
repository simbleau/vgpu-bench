# vgpu-bench
[![dependency status](https://deps.rs/repo/github/simbleau/vgpu-bench/status.svg)](https://deps.rs/repo/github/simbleau/vgpu-bench)
[![Build Status](https://app.travis-ci.com/simbleau/vgpu-bench.svg?branch=main)](https://app.travis-ci.com/simbleau/vgpu-bench)

A benching framework for third party vector graphics libraries, techniques, and renderers.

# Motivation

This project focuses on benching existing (usually hardware-accelerated) vector graphic utilities to export useful metrics and information. The information would accentuate provable theories about vector graphics as well as discover and highlight areas of friction and avant achievement.

This project is used as a framework to provide metrics for use in a wider scope of research regarding hardware-accelerated vector graphics.

# Sections

- [vgpu-bench](#vgpu-bench)
- [Motivation](#motivation)
- [Sections](#sections)
  - [Getting Started](#getting-started)
    - [Requirements](#requirements)
    - [Dependencies](#dependencies)
    - [Install](#install)
  - [Methodology](#methodology)
- [License](#license)

## Getting Started

#### Requirements:

 - **Operating System**
   - Ubuntu 20.04
   - Windows
 - **NVIDIA graphics card with minimum driver version:**
   - NVIDIA Turing architecture TU10x, TU11x - r440
   - NVIDIA Ampere architecture GA100 - r450
   - NVIDIA Ampere architecture GA100 MIG - r470 TRD1
   - NVIDIA Ampere architecture GA10x - r455

#### Dependencies
 - [Rust 1.58+](https://rustup.rs/)
 - [NVIDIA NSight-Systems](https://developer.nvidia.com/nsight-systems)
 - Ubuntu 20.04
   - `sudo apt install curl python3 python3-pip libgtk-3-dev`
   - `pip3 install matplotlib pandas numpy cairosvg`
 - Windows
   - You'll have to figure this out for yourself right now...

#### Install
 - Install dependencies above.
 - Clone repo
 - Clone [pathfinder](https://github.com/servo/pathfinder) repo wherever permissions are not an issue
 - Link pathfinder resources to same directory (e.g. `ln -s ~/pathfinder/resources ~/vgpu-bench/resources`)

## Methodology

The goal is to create an extensible and uniform benching framework which is capable of operating on a vector graphic test-suite. The framework is designed to be cross-platform and able to run on various hardware and GPU architecture (particularly NVIDIA).
Individual tests should be accentuated with GPU annotations and events (via [nvtx-rs](https://github.com/simbleau/nvtx-rs)) for fine-tuned metrics.

# License

This  project is dual-licensed under both [Apache 2.0](https://github.com/simbleau/convo/blob/main/LICENSE-APACHE) and [MIT](https://github.com/simbleau/convo/blob/main/LICENSE-MIT) licenses.
