# Getting Started

The project is decomposed into two parts:

* *Bonsai* is the compiler of the Java extension and is written in Rust.
* *Bonsai runtime* is the Java runtime used by the code compiled by bonsai.

We provide instructions to install both as well as their dependencies.

### Prerequisites

* [rustup](http://www.rustup.rs): `curl https://sh.rustup.rs -sSf | sh` (do not forget to source your profile).
* [Maven](https://maven.apache.org), it is usually available in the package manager of your distribution:
  1. MacOSX: `sudo brew install maven`
  2. Linux Ubuntu: `sudo apt-get install maven`

### Installation

You need to install both the compiler and the runtime using the `setup.py` script.

```sh
git clone https://github.com/ptal/bonsai.git
cd bonsai
python3 setup.py
```

We provide a manual installation procedure below in case the script did not work.

### Update

Update the compiler and runtime (without the external libraries such as Choco):

```sh
./update.sh
```

### Uninstallation

```sh
cargo uninstall bonsai
# Remove runtime in the Maven local database
rm ~/.m2/repository/bonsai
```

## Manual installation

Step-by-step installation of the bonsai compiler and its runtime.
Before attempting the manual installation, you should try the `setup.py` script as described above.

1. Install prerequisites: same as above (`rustup` and `Maven`).
2. Installing the bonsai compiler:
```sh
rustup override set nightly-2018-11-09
cargo install
```
You can verify everything is working by running `bonsai --help` in your terminal.
3. Installing Bonsai standard library:
The standard library provides several modules to ease the development of Bonsai application.

```sh
cd libstd # (inside the bonsai repository)
./install.sh
```
4. Installing the Bonsai runtime:
The Bonsai runtime has a dependency on [Choco](http://www.choco-solver.org) for constraint programming.
Choco will be downloaded automatically from the Maven central repository.
However, we need to install the Bonsai runtime in the local Maven database:
```sh
cd runtime/ # (inside the bonsai repository)
./install.sh
```

That's it! You should be ready to go to the next section.