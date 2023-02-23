# Ambient

[![Crates.io](https://img.shields.io/crates/v/ambient_api)](https://crates.io/crates/ambient_api)
[![docs.rs](https://img.shields.io/docsrs/ambient_api)](https://docs.rs/ambient_api)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AmbientRun/Ambient#license)
[![Discord](https://img.shields.io/discord/894505972289134632)](https://discord.gg/PhmPn6m8Tw)

Ambient is a runtime for building high-performance multiplayer games and 3D applications, powered by WebAssembly, Rust and WebGPU.

See our [announcement blog post](https://www.ambient.run/post/introducing-ambient) for more details.

## Features

- **Seamless networking**: Ambient is both your server and client. All you need to do is to build your server and/or client-side logic: the runtime handles synchronization of data for you.
- **Isolation**: Projects you build for Ambient are executed in isolation through the power of [WebAssembly](https://webassembly.org/) - so that if something crashes, it won’t take down your entire program. It also means that you can run untrusted code safely.
- **Data-oriented design**: The core data model of Ambient is an [entity component system](https://en.wikipedia.org/wiki/Entity_component_system) which each WASM module can manipulate.
- **Language-agnostic**: You will be able to build Ambient modules in any language that can compile to WebAssembly. At present, Rust is the only supported language, but we are working on expanding to other languages.
- **Single executable**: Ambient is a single executable which can run on Windows, Mac and Linux. It can act as a server or as a client.
- **Interoperability**: Ambient allows you to define custom components and "concepts" (collections of components). As long as your Ambient projects use the same components and concepts, they will be able to share data and interoperate, even if they have no awareness of each other.
- **Asset pipeline and streaming**: Ambient has an [asset pipeline](https://ambientrun.github.io/Ambient/asset_pipeline.html) that is capable of compiling multiple asset formats, including `.glb` and `.fbx`. The assets are always streamed over the network, so your clients will receive everything they need when they join.
- **Powerful renderer**: The Ambient renderer is GPU-driven, with both culling and level-of-detail switching being handled entirely by the GPU. By default, it uses [PBR](https://en.wikipedia.org/wiki/Physically_based_rendering). It also supports cascading shadow maps and instances everything that can be instanced.

See the [documentation](https://ambientrun.github.io/Ambient/) for a guide on how to get started, or browse the [examples](https://github.com/AmbientRun/Ambient/tree/main/guest/rust/examples).

## Installing

The easiest way to get Ambient is by downloading the latest release [here](https://github.com/AmbientRun/Ambient/releases).

For alternative installation options, go to the [documentation on installing](https://ambientrun.github.io/Ambient/installing.html).

## Roadmap

**_Note: Ambient is in an alpha stage and the API will be iterated on heavily. We are working towards a stable release._**

| Feature                 | Status | Notes                                                                                                                                                                                                                              |
| ----------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ECS                     | ✅     | Single-threaded.                                                                                                                                                                                                                   |
| WASM API                | ✅     | Rust is the only supported guest language right now, and WASM can only be used on the server. We are working on clientside WASM; see [this issue](https://github.com/AmbientRun/Ambient/issues/6).                                 |
| Multiplayer/networking  | ✅     | Multiplayer is server-authoritative without any prediction or compensation. See [this issue](https://github.com/AmbientRun/Ambient/issues/150).                                                                                    |
| GPU-driven renderer     | ✅     |                                                                                                                                                                                                                                    |
| FBX & glTF loading      | ✅     |                                                                                                                                                                                                                                    |
| Physics (through PhysX) | ✅     | Using PhysX 4.1. PhysX 5 support is tracked in [this issue](https://github.com/AmbientRun/Ambient/issues/155).                                                                                                                     |
| Animations              | ✅     |                                                                                                                                                                                                                                    |
| Skinmeshing             | ✅     |                                                                                                                                                                                                                                    |
| Shadow maps             | ✅     |                                                                                                                                                                                                                                    |
| Decals                  | ✅     |                                                                                                                                                                                                                                    |
| GPU culling and LoD     | ✅     |                                                                                                                                                                                                                                    |
| Multi-platform          | ✅     | Windows, Mac, and Linux so far. x86-64 and ARM64 are actively supported; other platforms may also work, but require testing.                                                                                                       |
| Run on Web              | 🚧     | See [this issue](https://github.com/AmbientRun/Ambient/issues/151).                                                                                                                                                                |
| Multithreading API      | 🚧     | Multithreading is already used internally, but we want to expose multithreading functionality within the WASM API. This may be explicit (i.e. task- or thread-spawning) or implicit (WASM modules being scheduled across threads). |
| UI API                  | 🚧     | A React-like UI library already exists in the repo, and we're working on exposing it through the WASM API. See [this issue](https://github.com/AmbientRun/Ambient/issues/40).                                                      |
| Custom shaders          | 🚧     | Custom shaders are supported by the renderer, but are not yet exposed in the API. See [this issue](https://github.com/AmbientRun/Ambient/issues/98).                                                                               |
| Hot-reloading assets    | 🚧     | See [this issue](https://github.com/AmbientRun/Ambient/issues/12).                                                                                                                                                                 |
| Audio                   | 🚧     | Audio is supported, but not currently exposed. See [this issue](https://github.com/AmbientRun/Ambient/issues/76).                                                                                                                  |
| ECS save/load           | 🚧     | For loading, [see this issue](https://github.com/AmbientRun/Ambient/issues/71).                                                                                                                                                    |

## Examples

Each example in the [examples](./guest/rust/examples/) directory can be run with Ambient as both client and server:

- `cd guest/rust/examples/tictactoe`
- `ambient run`

Every example can also be run server-only. To do so:

- `cd guest/rust/examples/tictactoe`
- `ambient serve`

This will start a server that other people, including yourself, can join (assuming that ports 8999 and 9000 are forwarded):

- `ambient join [IP_OF_SERVER]`

Note that content is always streamed, so the only thing the joining user requires is Ambient itself to join the session.

## Contributing

We welcome community contributions to this project.

Please talk with us on Discord beforehand if you'd like to contribute a larger piece of work.

## License (MIT)

Ambient is licensed under MIT. See the [LICENSE](./LICENSE.md).

