# bonsai

[![ptal on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/ptal/bonsai.png
[travis]: https://travis-ci.org/ptal/bonsai

Bonsai is a programming language on top of Java inspired by synchronous programming and aims at be used to solve Constraint Satisfaction Problems (CSP). The project is decomposed into two parts:

* *Bonsai* is the compiler of the Java extension and is written in Rust.
* *ChocoCubes* is the Java runtime used by the code compiled by bonsai.

# Getting Started

## Prerequisites

* [rustup](http://www.rustup.rs): `curl https://sh.rustup.rs -sSf | sh`
* [Maven](https://maven.apache.org), it is usually available in the package manager of your distribution:
  1. MacOSX: `sudo brew install maven`
  2. Linux Ubuntu: `sudo apt-get install maven`

## Installation

You need to install both the compiler and the ChocoCubes runtime.

```sh
git clone https://github.com/ptal/bonsai.git
cd bonsai
python3 setup.py
```

We provide a [manual installation procedure](manual-installation.md) in case the script did not work.

### Update

We only update the compiler and runtime (and not the dependencies):

```sh
.\update.sh
```

### Uninstallation

```sh
cargo uninstall bonsai
# Remove runtime in the Maven local database
rm ~/.m2/repository/inria
rm ~/.m2/repository/bonsai
```

Remove the following export line in your profile:

1. Linux in `~/.bashrc`: `export LD_LIBRARY_PATH=\$LD_LIBRARY_PATH:~/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib`
2. MacOSX in `~/.bash_profile`: `export DYLD_LIBRARY_PATH=\$DYLD_LIBRARY_PATH:~/.multirust/toolchains/x86_64-apple-darwin/lib`

# Example

```sh
cd bonsai/examples/NQueens
mvn compile
mvn exec:java -Dexec.mainClass="chococubes.example.NQueens"
```

## Into an existing project

Copy this project and use it as a template to start your own! If you already have a running project, don't forget to add to the `pom.xml` the following dependencies (note that Choco is available on the Maven central repository so you don't need to install it):

```
<dependency>
  <groupId>org.choco-solver</groupId>
  <artifactId>choco-solver</artifactId>
  <version>4.0.0</version>
</dependency>
<dependency>
  <groupId>inria.meije.rc</groupId>
  <artifactId>SugarCubes</artifactId>
  <version>4.0.0a5</version>
</dependency>
<dependency>
  <groupId>bonsai</groupId>
  <artifactId>ChocoCubes</artifactId>
  <version>1.0</version>
</dependency>
```
