# Manual installation procedure

Step-by-step installation of the bonsai compiler and its runtime. Before attempting the manual installation, you should try the `setup.py` script as described in [README.md](README.md).

## Prerequisites

See [README.md](README.md)

## Installing the bonsai compiler

```sh
rustup override set nightly-2017-04-27
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

The Bonsai runtime has two dependencies: [SugarCubes](http://jeanferdysusini.free.fr/index.php?action=SC) for synchronous and reactive programming and [Choco](http://www.choco-solver.org) for constraint programming.

1. Installing SugarCubes in the local Maven database:
  ```sh
  curl http://jeanferdysusini.free.fr/v4.0/SugarCubesv4.0.0a5.jar > /tmp/SugarCubesv4.0.0a5.jar
  mvn install:install-file -DgroupId=inria.meije.rc -DartifactId=SugarCubes -Dversion=4.0.0a5 -Dpackaging=jar -Dfile=/tmp/SugarCubesv4.0.0a5.jar
  ```
2. Installing Bonsai runtime in the local Maven database:
  ```sh
  cd runtime/ # (inside the bonsai repository)
  ./install.sh
  ```
