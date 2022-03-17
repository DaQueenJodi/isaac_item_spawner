
## Building
#### Prerequisites

* [Rust](https://www.rust-lang.org/install.html) - make sure rustup, cargo, and rustc (preferrably nightly) are installed.
* [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

Install [cargo skyline](https://github.com/jam1garner/cargo-skyline).
```bash
cargo install cargo-skyline
```
```bash
git clone https://github.com/DaQueenJodi/isaac_item_spawner.git
cd isaac_item_spawner
cargo skyline release
```
your files will then appear in `target/release.zip`
## Installation
#### Prerequisites
* [Atmosphere](https://github.com/Atmosphere-NX/Atmosphere)
* A supported version of TBOI - currently only base AB+ 

Copy subskd9 and main.npdm to /atmosphere/contents/010021C000B6A000/exefs

Copy the releases.zip file to /atmosphere/contents/010021C000B64A000/romfs/plugins

or alternatively, just run `cargo skyline install` from the project directory (see the Building section)
