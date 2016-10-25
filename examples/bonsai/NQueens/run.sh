#!/bin/sh

mvn compile
mvn exec:java -Dexec.mainClass="chococubes.example.NQueens"
