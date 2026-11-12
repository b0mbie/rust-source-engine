# Rust Source Engine
This repository is dedicated to providing implementations
for interacting with the Source 1 engine in Rust.
See the crates in the `examples/` directory for potential use cases.

> [!WARNING]
> Due to the large scope of this project,
> the crates are likely to be highly unstable.
> There may be problems with soundness, non-existent safe APIs, or
> incomplete definitions that can cause *Undefined Behavior*
> (with the potential to silently corrupt program state).
> **You have been warned!**

## License
Please note that, as it stands right now,
the circumstances of this project existing are rather unique
and it is unclear to me what can and cannot be allowed for users -
thus, this project is *All Rights Reserved* until further notice.
(I plan to re-license it to allow more permissive usage in the future.)

## Overview
The project consists of a workspace with crates that provide access to some functionality of the Source 1 engine.
It was initially developed to allow for writing Valve Server Plugins for the Source 1 engine,
though it *may or may not* be possible to use the code provided for more ambitious projects.
`#![no_std]` features are used wherever possible, providing both safe and `unsafe` APIs.
