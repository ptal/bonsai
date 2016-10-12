# bonsai

[![ptal on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/ptal/bonsai.png
[travis]: https://travis-ci.org/ptal/bonsai

Bonsai is a programming language inspired by synchronous programming and aims at be used to solve Constraint Satisfaction Problems (CSP).

The bonsai compiler is in development. We are currently working on the target Java library, named ChocoCubes, using [SugarCubes](http://jeanferdysusini.free.fr/index.php?action=SC) for the reactive part and [Choco](http://www.choco-solver.org) for the constraint programming part. An example can be find [here](https://github.com/ptal/bonsai/blob/master/examples/NQueens/src/main/java/chococubes/example/NQueens.java), the code in comment the bonsai programming language that should be compiled to the code in Java.

## Building and executing a first example

This tutorial describe how to set up a project with ChocoCubes, a Java library for synchronous programming based on SugarCubes and the Choco constraint library. Here the steps:

1. Download and install [Maven](https://maven.apache.org).
2. Download the JAR file of [SugarCubes V4](http://jeanferdysusini.free.fr/v4.0/SugarCubesv4.0.0a5.jar).
3. Copy the following line in your terminal for locally installing SugarCubes:

  ```
  mvn install:install-file -DgroupId=inria.meije.rc -DartifactId=SugarCubes -Dversion=4.0.0a5 -Dpackaging=jar -Dfile=SugarCubesv4.0.0a5.jar
  ```

4. Build and package ChocoCubes (this library) as a .jar:

  ```
  git clone https://github.com/ptal/bonsai.git
  cd bonsai/ChocoCubes/
  mvn package
  ```

  The .jar is in the repository `target`. We can now install this version in our Maven repository so we can use it later in other project:

  ```
  mvn install:install-file -DgroupId=bonsai -DartifactId=ChocoCubes -Dversion=1.0 -Dpackaging=jar -Dfile=target/ChocoCubes-1.0-SNAPSHOT.jar
  ```

5. We run a simple example solving the NQueens problem in `bonsai/examples/NQueens`:

  ```
  mvn compile
  mvn exec:java -Dexec.mainClass="chococubes.example.NQueens"
  ```

6. Copy this project and use it as a template to start your own! If you already have a running project, don't forget to add to the `pom.xml` the following dependencies (note that Choco is available on the Maven central repository so you don't need to install it):

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
