<h1 align="center">
  <br/>
  names
  <br/>
</h1>

<h4 align="center">
  Random name generator for Rust
</h4>

Maintained fork of [fnichol/names](https://github.com/fnichol/names).
This fork uses Rust 2024 with an MSRV of 1.85. Published crates, pre-built
binaries, installers, and Docker images linked below are upstream artifacts;
use the Git dependency or build this fork from source for its changes.

|                  |                                                                                          |
| ---------------: | ---------------------------------------------------------------------------------------- |
|               CI | [![CI Status][badge-ci-overall]][ci] |
|   Latest Version | [![Latest version][badge-version]][crate]                                                |
|    Documentation | [![Documentation][badge-docs]][docs]                                                     |
|  Crate Downloads | [![Crate downloads][badge-crate-dl]][crate]                                              |
| GitHub Downloads | [![Github downloads][badge-github-dl]][github-releases]                                  |
|     Docker Pulls | [![Docker pulls][badge-docker-pulls]][docker]                                            |
|          License | [![Crate license][badge-license]][github]                                                |

<details>
<summary><strong>Table of Contents</strong></summary>

<!-- toc -->

- [CLI](#cli)
  - [Usage](#usage)
  - [Installation](#installation)
    - [install.sh (Pre-Built Binaries)](#installsh-pre-built-binaries)
    - [GitHub Releases (Pre-Built Binaries)](#github-releases-pre-built-binaries)
    - [Docker Image](#docker-image)
    - [Cargo Install](#cargo-install)
    - [From Source](#from-source)
- [Library](#library)
  - [Usage](#usage-1)
  - [Examples](#examples)
    - [Example: painless defaults](#example-painless-defaults)
    - [Example: with custom dictionaries](#example-with-custom-dictionaries)
- [CI Status](#ci-status)
- [Development](#development)
- [Code of Conduct](#code-of-conduct)
- [Issues](#issues)
- [Contributing](#contributing)
- [Release History](#release-history)
- [Authors](#authors)
- [License](#license)

<!-- tocstop -->

</details>

## CLI

### Usage

Simple! Run without any parameters, you get a name:

```console
> names
selfish-change
```

Need more? Tell it how many:

```console
> names 10
rustic-flag
nondescript-crayon
picayune-map
elderly-cough
skinny-jeans
neat-rock
aware-sponge
psychotic-coast
brawny-event
tender-oatmeal
```

Not random enough? How about adding a 4-number pad:

```console
> names --number 5
imported-rod-9680
thin-position-2344
hysterical-women-5647
volatile-pen-9210
diligent-grip-4520
```

If you're ever confused, at least there's help:

```console
> names --help
A random name generator with results like "delirious-pail"

Usage: names [OPTIONS] [AMOUNT]

Arguments:
  [AMOUNT]  Number of names to generate [default: 1]

Options:
  -n, --number   Adds a random number to the name(s)
  -h, --help     Print help
  -V, --version  Print version
```

### Installation

#### install.sh (Pre-Built Binaries)

An installer is provided at <https://fnichol.github.io/names/install.sh> which
installs a suitable pre-built binary for common systems such as Linux, macOS,
Windows, and FreeBSD. It can be downloaded and run locally or piped into a shell
interpreter in the "curl-bash" style as shown below. Note that if you're opposed
to this idea, feel free to check some of the alternatives below.

To install the latest release for your system into `$HOME/bin`:

```sh
curl -sSf https://fnichol.github.io/names/install.sh | sh
```

When the installer is run as `root` the installation directory defaults to
`/usr/local/bin`:

```sh
curl -sSf https://fnichol.github.io/names/install.sh | sudo sh
```

A [nightly] release built from `HEAD` of the main branch is available which can
also be installed:

```sh
curl -sSf https://fnichol.github.io/names/install.sh \
    | sh -s -- --release=nightly
```

For a full set of options, check out the help usage with:

```sh
curl -sSf https://fnichol.github.io/names/install.sh | sh -s -- --help
```

#### GitHub Releases (Pre-Built Binaries)

Each release comes with binary artifacts published in [GitHub
Releases][github-releases]. The `install.sh` program downloads its artifacts
from this location so this serves as a manual alternative. Each artifact ships
with MD5 and SHA256 checksums to help verify the artifact on a target system.

#### Docker Image

A minimal image ships with each release (including a [nightly] built version
from `HEAD` of the main branch) published to [Docker Hub][docker]. The
entrypoint invokes the binary directly, so any arguments to `docker run` will be
passed to the program. For example, to display the full help usage:

```sh
docker run fnichol/names --help
```

#### Cargo Install

If [Rust](https://rustup.rs/) is installed on your system, then installing with
Cargo is straight forward with:

```sh
cargo install names
```

#### From Source

To install from source, you can clone the Git repository, build with Cargo and
copy the binary into a destination directory. This will build the project from
the latest commit on the main branch, which may not correspond to the latest
stable release:

```console
> git clone https://github.com/shunichironomura/names.git
> cd names
> cargo build --release
> cp ./target/release/names /dest/path/
```

---

## Library

This crate provides a generate that constructs random name strings suitable for
use in container instances, project names, application instances, etc.

The name `Generator` implements the `Iterator` trait so it can be used with
adapters, consumers, and in loops.

### Usage

This crate is [on crates.io](https://crates.io/crates/names) and can be used by
adding `names` to your dependencies in your project's `Cargo.toml` file:

```toml
[dependencies]
names = { version = "0.15.0", default-features = false }
```

For browser or Node.js builds targeting `wasm32-unknown-unknown`, also enable
the `wasm_js` feature. This selects JavaScript randomness via `getrandom`.
Native builds do not need this feature. This fork requires Rust 1.85 or later.

To use this fork directly, replace the version with a Git dependency:

```toml
[dependencies]
names = { git = "https://github.com/shunichironomura/names", rev = "<commit SHA>", default-features = false, features = ["wasm_js"] }
```

### Examples

#### Example: painless defaults

The easiest way to get started is to use the default `Generator` to return a
name:

```rust
use names::Generator;

let mut generator = Generator::default();
println!("Your project is: {}", generator.next().unwrap());
// #=> "Your project is: rusty-nail"
```

If more randomness is required, you can generate a name with a trailing 4-digit
number:

```rust
use names::{Generator, Name};

let mut generator = Generator::with_naming(Name::Numbered);
println!("Your project is: {}", generator.next().unwrap());
// #=> "Your project is: pushy-pencil-5602"
```

#### Example: with custom dictionaries

If you would rather supply your own custom adjective and noun word lists, you
can provide your own by supplying 2 string slices. For example, this returns
only one result:

```rust
use names::{Generator, Name};

let adjectives = &["imaginary"];
let nouns = &["roll"];
let mut generator = Generator::new(adjectives, nouns, Name::default());

assert_eq!("imaginary-roll", generator.next().unwrap());
```

## CI Status

[GitHub Actions][ci] runs on pull requests and pushes to `main`:

- Formatting, Clippy, and documentation with warnings denied.
- Default, library-only, and all-feature tests on Linux, macOS, and Windows,
  including doctests.
- Every feature on the MSRV declared in `Cargo.toml`.
- Browser/Node.js and WASI library compilation.
- LLVM coverage, uploaded as the `coverage-lcov` workflow artifact (no secrets needed).
- Dependency auditing, actionlint, and zizmor workflow security checks.

The **CI gate** job aggregates all results for branch protection. FreeBSD and
the old upstream release-target matrix are not currently tested by this fork.

## Development

Install [Rust via rustup](https://rustup.rs/); `rust-toolchain.toml` pins the
development toolchain independently of the MSRV. CI and local commands use the
committed lockfile. Renovate maintains dependencies, action digests, the
toolchain, and the lockfile.

Install [just](https://just.systems/) and the optional Rust tools:

```sh
cargo install --locked just cargo-hack cargo-audit cargo-llvm-cov
```

For workflow checks, also install [actionlint](https://github.com/rhysd/actionlint)
and [zizmor](https://docs.zizmor.sh/installation/).

```sh
just lint       # Formatting, Clippy, and docs
just test       # Default, library-only, and all-feature tests
just msrv       # Check every feature on Cargo.toml's rust-version
just wasm       # Install Wasm targets and check browser/Node.js and WASI
just audit      # RustSec audit, including warnings
just workflows # actionlint and zizmor
just check      # All of the above
just coverage   # Local HTML coverage report
```

The recipes are convenience wrappers; Cargo commands can also be run directly
(on Windows, use `just --show <recipe>` to see them). Coverage recipes require
`llvm-tools-preview`, installable with `rustup component add llvm-tools-preview`.

The legacy Cirrus, Bors, and cargo-make setup has been retired. This fork does
not automatically publish crates, GitHub release artifacts, or Docker images.

## Code of Conduct

This project adheres to the Contributor Covenant [code of
conduct][code-of-conduct]. By participating, you are expected to uphold this
code. Please report unacceptable behavior to fnichol@nichol.ca.

## Issues

If you have any problems with or questions about this project, please contact us
through a [GitHub issue][issues].

## Contributing

You are invited to contribute to new features, fixes, or updates, large or
small; we are always thrilled to receive pull requests, and do our best to
process them as fast as we can.

Before you start to code, we recommend discussing your plans through a [GitHub
issue][issues], especially for more ambitious contributions. This gives other
contributors a chance to point you in the right direction, give you feedback on
your design, and help you find out if someone else is working on the same thing.

## Release History

See the [changelog] for a full release history.

## Authors

Created by [Fletcher Nichol][fnichol] (<fnichol@nichol.ca>).
This fork is maintained by [Shunichiro Nomura](https://github.com/shunichironomura).

## License

Licensed under the MIT license ([LICENSE.txt][license]).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MIT license, shall be
licensed as above, without any additional terms or conditions.

[badge-ci-overall]: https://github.com/shunichironomura/names/actions/workflows/ci.yaml/badge.svg
[badge-crate-dl]: https://img.shields.io/crates/d/names.svg?style=flat-square
[badge-docker-pulls]:
  https://img.shields.io/docker/pulls/fnichol/names.svg?style=flat-square
[badge-docs]: https://docs.rs/names/badge.svg?style=flat-square
[badge-github-dl]:
  https://img.shields.io/github/downloads/fnichol/names/total.svg
[badge-license]: https://img.shields.io/crates/l/names.svg?style=flat-square
[badge-version]: https://img.shields.io/crates/v/names.svg?style=flat-square
[changelog]: CHANGELOG.md
[ci]: https://github.com/shunichironomura/names/actions/workflows/ci.yaml
[code-of-conduct]: CODE_OF_CONDUCT.md
[commonmark]: https://commonmark.org/
[crate]: https://crates.io/crates/names
[docker]: https://hub.docker.com/r/fnichol/names
[docs]: https://docs.rs/names
[fnichol]: https://github.com/fnichol
[github]: https://github.com/shunichironomura/names
[github-releases]: https://github.com/fnichol/names/releases
[issues]: https://github.com/shunichironomura/names/issues
[license]: LICENSE.txt
[nightly]: https://github.com/fnichol/names/releases/tag/nightly
