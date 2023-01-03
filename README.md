# Simple PSP Framework

A simple, Rust-based, modular framework for Sony's Playstation Portable.

## Table of Contents

- [Simple PSP Framework](#simple-psp-framework)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Building to PSP Executable](#building-to-psp-executable)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)

## Installation

To properly compile any projects utilizing SPSPF you **must use a Rust nightly** channel version equal or later than `2022-09-04` and the `rust-src` component.


You can use Rust nightly for all your projects by using the following command:

```bash
$ rustup default nightly
```

You can also utilize the method displayed in the [examples](https://github.com/originals1n/spspf-rs/examples) and only utilize Rust nightly in a single project by adding a file to your project's main directory with the name `rust-toolchain.toml` and the following content:

```toml
[toolchain]
channel = "nightly"
```

To install `rust-src` you can use the following command:
```bash
$ rustup component add rust-src
```

Apart from the Rust nightly dated `2022-09-04` you must also install the crate `cargo-psp` in order to compile for the platform. You can do that with the following command:

```bash
$ cargo install cargo-psp
```

## Usage

```toml
# Cargo.toml
[dependencies]
psp = "0.3.2"
spspf-core = "0.1.0"
spspf-graphics = "0.1.0"
spspf-audio = "0.1.0"
```

Check the [SPSPF-Template](https://github.com/originals1n/spspf-rs/examples/spspf-template) for a basic setup utilizing most features available in SPSPF.

## Building to PSP Executable

The PSP utilizes a specific executable format, and to allow building Rust software to the platform you can use the `cargo-psp` crate written by [overdrivenpotato](https://github.com/overdrivenpotato) like this.

On the root directory of your project, type the following:

```bash
$ cargo psp
```

`cargo-psp` will do its job and output the `EBOOT.PBP` (PSP executable format) to the following directory:
`[PROJECT_ROOT]/target/mipsel-sony-psp/debug/EBOOT.PBP`.

Just copy this file to your PSP's memory stick in a new folder inside `PSP/GAME` and execute it from your XMB menu.

If you don't have a PSP with CFW installed you must use a tool called `PRXEncrypter` to sign the executable and make it able to be ran under an OFW PSP.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://github.com/originals1n/spspf-rs/LICENSE.md)

## Acknowledgements
- overdrivenpotato for the [rust-psp](https://github.com/overdrivenpotato/rust-psp) that brought the PSP to the Rust world.
- pspdev team for [pspsdk](https://github.com/pspdev/pspsdk).
- [PSP Homebrew Community](https://discord.gg/gMsKGzXts4) on Discord for the support in the development of this project.