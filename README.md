# Rust Source Engine
This repository is dedicated to providing implementations
for interacting with the Source 1 engine in Rust.

> [!WARNING]
> Due to the large scope of this project,
> the crates are likely to be highly unstable
> while the C++ definitions are ported to Rust.
> There may also be problems with soundness, or non-existent safe APIs.

## Overview
The project consists of a workspace with crates that provide access to some functionality of the Source 1 engine.
It was initially developed to allow for writing Valve Server Plugins for the Source 1 engine,
though it *may or may not* be possible to use the code provided for more ambitious projects.
`#![no_std]` features are used wherever possible, providing both safe and `unsafe` APIs.

Valve Server (or Client!) Plugin developers will want to look at
the `rse-plugin` crate and the `test-plugin` as a reference.
