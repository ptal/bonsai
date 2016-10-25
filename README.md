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

The following command will compile and execute the NQueens problem as described in the file [NQueens.bonsai.java](examples/bonsai/NQueens/src/main/java/chococubes/example/NQueens.bonsai.java):

```sh
cd examples/bonsai/NQueens
mvn compile
mvn exec:java -Dexec.mainClass="chococubes.example.NQueens"
```

Copy this project and create as much bonsai file (`.bonsai.java`) as you want. For each file created add an `execution` section in your `pom.xml`:

```xml
<plugin>
  <groupId>org.codehaus.mojo</groupId>
  <artifactId>exec-maven-plugin</artifactId>
  <version>1.5.0</version>
  <executions>
    <execution>
      <id>SomeFile.bonsai.java</id>
      ...
    </execution>
    <!-- Add an execution section as follow: -->
    <execution>
      <id>NQueens.bonsai.java</id>
      <phase>generate-sources</phase>
      <goals>
        <goal>exec</goal>
      </goals>
      <configuration>
        <executable>bonsai</executable>
        <arguments>
          <argument>--main</argument>
          <argument>-o ${project.build.directory}/generated-sources/bonsai/NQueens.java</argument>
          <argument>${project.build.sourceDirectory}/chococubes/example/NQueens.bonsai.java</argument>
        </arguments>
      </configuration>
    </execution>
  </executions>
```

Do not forget to modify the `<id>` and the names of the files in the `<argument>`.
