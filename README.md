# bonsai

[![ptal on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/ptal/bonsai.png
[travis]: https://travis-ci.org/ptal/bonsai

Bonsai is a programming language on top of Java inspired by synchronous programming and aims at be used to solve Constraint Satisfaction Problems (CSP).
This is a work-in-progress to implement the language formally described in [this dissertation](http://hyc.io/papers/dissertation-talbot.pdf).

The project is decomposed into two parts:

* *Bonsai* is the compiler of the Java extension and is written in Rust.
* *Bonsai runtime* is the Java runtime used by the code compiled by bonsai.

# Getting Started

## Prerequisites

* [rustup](http://www.rustup.rs): `curl https://sh.rustup.rs -sSf | sh` (do not forget to source your profile).
* [Maven](https://maven.apache.org), it is usually available in the package manager of your distribution:
  1. MacOSX: `sudo brew install maven`
  2. Linux Ubuntu: `sudo apt-get install maven`

## Installation

You need to install both the compiler and the runtime using the `setup.py` script.

```sh
git clone https://github.com/ptal/bonsai.git
cd bonsai
python3 setup.py
```

We provide a [manual installation procedure](manual-installation.md) in case the script did not work.

### Update

Update the compiler and runtime (without updating SugarCubes and Choco libraries):

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

# Example

The following command will compile and execute the NQueens problem as described in the file [NQueens.bonsai.java](examples/bonsai/NQueens/src/main/java/bonsai/examples/NQueens.bonsai.java):

```sh
cd examples/bonsai/NQueens
mvn compile
mvn exec:java -Dexec.mainClass="bonsai.examples.NQueens"
```

Copy this project and create as much bonsai file (`.bonsai.java`) as you want. Do not forget to modify the `<id>` and the names of the files in the `<argument>`. Also, note that it includes the Bonsai standard library (in `/libstd`), you can modify its path in `<argument>--lib=...</argument>`.
