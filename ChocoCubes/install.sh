#!/bin/sh

mvn compile
mvn package
mvn install:install-file -DgroupId=bonsai -DartifactId=ChocoCubes -Dversion=1.0 -Dpackaging=jar -Dfile=target/ChocoCubes-1.0-SNAPSHOT.jar
