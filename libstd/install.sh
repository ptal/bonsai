#!/bin/sh

mvn clean
mvn package
mvn install:install-file -DgroupId=bonsai -DartifactId=libstd -Dversion=1.0 -Dpackaging=jar -Dfile=target/libstd-1.0-SNAPSHOT.jar
