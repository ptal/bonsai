#!/bin/sh

mvn clean
mvn package
mvn install:install-file -DgroupId=bonsai -DartifactId=runtime -Dversion=1.0 -Dpackaging=jar -Dfile=target/runtime-1.0-SNAPSHOT.jar
