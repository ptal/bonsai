# Manual installation procedure

Step-by-step installation of the bonsai compiler and its runtime. Before attempting the manual installation, you should try the `setup.py` script as described in [README.md](README.md).

## Prerequisites

See [README.md](README.md)

## Installing the bonsai compiler

```sh
rustup override set nightly-2018-08-18
cargo install
```

You can verify everything is working by running `bonsai --help` in your terminal.

## Installing Bonsai standard library

The standard library provides several modules to ease the development of Bonsai application.

```sh
cd libstd # (inside the bonsai repository)
./install.sh
```

## Installing the Bonsai runtime

The Bonsai runtime has a dependency on [Choco](http://www.choco-solver.org) for constraint programming.
Choco will be downloaded automatically from the Maven central repository.
However, we need to install the Bonsai runtime in the local Maven database:
```sh
cd runtime/ # (inside the bonsai repository)
./install.sh
```
