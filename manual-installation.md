# Manual installation procedure

Step-by-step installation of the bonsai compiler and its runtime ChocoCubes. You should try the `setup.py` script before as described in the `README.md`.

## Prerequisites

See [README.md](README.md)

## Installing the bonsai compiler

```sh
rustup override set nightly-2016-10-21
cargo install
```

There is a bug where `bonsai` do not link statically to the standard library, therefore you need to specify the link to the library as follow:

1. Linux: `echo "export LD_LIBRARY_PATH=\$LD_LIBRARY_PATH:~/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib" >> ~/.bashrc && source ~/.bashrc`
2. MacOSX: `echo "export DYLD_LIBRARY_PATH=\$DYLD_LIBRARY_PATH:~/.multirust/toolchains/x86_64-apple-darwin/lib" >> ~/.bash_profile && source ~/.bash_profile`

You can verify everything is working by running `bonsai --help` in your terminal.

*Alternatively* (without the export), you can run `bonsai` with `cargo run -- --help`.

## Installing the ChocoCubes runtime

The ChocoCubes runtime has two dependencies: [SugarCubes](http://jeanferdysusini.free.fr/index.php?action=SC) for synchronous and reactive programming and [Choco](http://www.choco-solver.org) for constraint programming.

1. Installing SugarCubes in the local Maven database:
```sh
curl http://jeanferdysusini.free.fr/v4.0/SugarCubesv4.0.0a5.jar > /tmp/SugarCubesv4.0.0a5.jar
mvn install:install-file -DgroupId=inria.meije.rc -DartifactId=SugarCubes -Dversion=4.0.0a5 -Dpackaging=jar -Dfile=/tmp/SugarCubesv4.0.0a5.jar
```
2. Installing ChocoCubes in the local Maven database:
```sh
cd ChocoCubes/ # (inside the bonsai repository)
mvn package
mvn install:install-file -DgroupId=bonsai -DartifactId=ChocoCubes -Dversion=1.0 -Dpackaging=jar -Dfile=target/ChocoCubes-1.0-SNAPSHOT.jar
```
