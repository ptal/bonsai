# Companion guide (PPDP19)

* Reference: P. Talbot, “Spacetime Programming: A Synchronous Language for Composable Search Strategies,” in Proceedings of the 21st ACM International Symposium on Principles and Practice of Declarative Programming (PPDP 2019), Porto, Portugal, 2019.
* The paper is available [here](http://hyc.io/papers/ppdp2019.pdf).
* Tagged version of the language used in the paper on [Github](https://github.com/ptal/bonsai/tree/PPDP19).

This supplementary material gives instructions to compile and run the examples and benchmarks presented in the paper.

If you want to replicate any benchmark and running examples, first install the compiler and runtime as follows:

```
git clone https://github.com/ptal/bonsai.git
cd bonsai
git checkout PPDP19
python3 setup.py
```

In case of problems, please go to the [Getting Started](getting-started.html) section (but do not forget to switch to the `PPDP19` branch).

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

There are about 200 tests, ranging from the static analysis of the compiler ([compile-fail](https://github.com/ptal/bonsai/tree/master/data/test/compile-fail) and [compile-pass](https://github.com/ptal/bonsai/tree/master/data/test/compile-pass)), to the runtime behavior ([run-pass](https://github.com/ptal/bonsai/tree/master/data/test/run-pass)), and the correctness of the lattice library.
