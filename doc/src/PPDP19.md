# Companion guide to the paper submitted to PPDP19

This supplementary material gives instructions to compile and run the examples and benchmarks presented in the paper submitted to PPDP19.

If you want to replicate any benchmark and running examples, please go through the [Getting Started](getting-started.html) section first.

## Demo of the examples in the paper

We provide a demonstration of the programs given in the paper in the directory [PPDP19](https://github.com/ptal/bonsai/tree/PPDP19/examples/bonsai/PPDP19).
After installing spacetime, you can simply type:

```sh
cd examples/bonsai/PPDP19
./run.sh
```

## Additional examples of strategies

Examples are provided in the [standard library](https://github.com/ptal/bonsai/tree/master/libstd/src/main/java/bonsai) of spacetime.
In particular, we have the following:

* [Depth-bounded discrepancy search (DDS)](https://github.com/ptal/bonsai/blob/PPDP19/libstd/src/main/java/bonsai/strategies/DDS.bonsai.java)
* [Improved Limited Discrepancy Search (ILDS)](https://github.com/ptal/bonsai/blob/PPDP19/libstd/src/main/java/bonsai/strategies/ILDS.bonsai.java)
* Highest and lowest discrepancy first variations of LDS can be obtained by changing the queueing strategy from `StackLR` to `StackRL`.
* [Branch and bound search (BAB)](https://github.com/ptal/bonsai/blob/PPDP19/libstd/src/main/java/bonsai/cp/MaximizeBAB.bonsai.java)

## How to run the benchmark

After installing spacetime, you can simply type:

```sh
cd benchmark
./run.sh
```

The file [Benchmark.java](https://github.com/ptal/bonsai/blob/master/benchmark/src/main/java/benchmark/Benchmark.java) contains some parameters that can be tweaked such as the size of the instances.

## Tests

The compiler and runtime of spacetime are well tested, you can run the tests of the compiler with:

```sh
cargo test
cd runtime
mvn test
```

There are about 200 tests, ranging from the static analysis of the compiler ([compile-fail](https://github.com/ptal/bonsai/tree/master/data/test/compile-fail) and [compile-pass](https://github.com/ptal/bonsai/tree/master/data/test/compile-pass)) as well as the runtime behavior ([run-pass](https://github.com/ptal/bonsai/tree/master/data/test/run-pass)) to the correctness of the lattice library.
