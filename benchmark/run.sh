#!/bin/sh

mvn clean &&
mvn compile &&
mvn exec:java -Dexec.mainClass="benchmark.Benchmark"
