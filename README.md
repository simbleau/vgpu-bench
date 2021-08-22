# vgpu-bench
[![dependency status](https://deps.rs/repo/github/simbleau/vgpu-bench/status.svg)](https://deps.rs/repo/github/simbleau/vgpu-bench)

A benching framework for third party vector graphics libraries, techniques, and renderers.

# Motivation

This project focuses on benching existing (usually hardware-accelerated) vector graphic utilities to export useful metrics and information. The information would accentuate provable theories about vector graphics as well as discover and highlight areas of friction and avant achievement.

This project is used as a framework to provide metrics for use in a wider scope of research regarding hardware-accelerated vector graphics.

# Sections

- [vgpu-bench](#vgpu-bench)
- [Motivation](#motivation)
- [Sections](#sections)
  - [Methodology](#methodology)
- [License](#license)

## Methodology

The goal is to create an extensible and uniform benching framework which is capable of operating on a vector graphic test-suite. The framework is designed to be cross-platform and able to run on various hardware and GPU architecture (particularly NVIDIA).
Individual tests should be accentuated with GPU annotations and events (via [nvtx-rs](https://github.com/simbleau/nvtx-rs)) for fine-tuned metrics.

# License

This  project is dual-licensed under both [Apache 2.0](https://github.com/simbleau/convo/blob/main/LICENSE-APACHE) and [MIT](https://github.com/simbleau/convo/blob/main/LICENSE-MIT) licenses.
