# Spacetime Programming

Spacetime programming is a programming language on top of Java to describe search strategy when exploring a state-space, for example in constraint satisfaction problems.
Spacetime started as a research project and is formally described in [this dissertation](http://hyc.io/papers/dissertation-talbot.pdf).
A Java project integrating spacetime code can be set up in 5 minutes with the Maven build system.

Spacetime is based on two main paradigms: [synchronous programming](https://en.wikipedia.org/wiki/Synchronous_programming_language) and [concurrent constraint programming](https://en.wikipedia.org/wiki/Concurrent_constraint_logic_programming) (CCP).
These two paradigms are not mainstream and it is very likely that you did not hear about these two paradigms before.
In this tutorial, we walk you through the syntax and semantics of spacetime with various examples.
Our goal is to keep the tutorial accessible to programmers and computer scientists, and to give you an idea of the kind of problems this language can solve.

Independently of your background, a first step is to read the [Getting Started](getting-started.md) chapter.
We introduce the concepts of [Synchronous Programming](synchronous-programming.md) and the syntax of spacetime through several small examples.
We make sure the build system is working out for you.
Secondly, we introduce some novelties of [Spacetime Programming](learn-spacetime.md) with examples stemming from constraint satisfaction problems such as Sudoku.
We next consider the more advanced concept of [Universe](universe.md) which is useful to design restarting search strategies.
Finally, we go through a full application of this language by designing a interactive search strategy for an [application of musical composition](application-composition.md).

The code is available on [github](https://github.com/ptal/bonsai).

### Syntax cheat sheet

`p` and `q` are statements, `e` is an expression, `x` a variable identifier and `st` is a spacetime attribute (either `single_space`, `single_time` and `world_line`).

| Statements      | Description |
| --------------- | ----------- |
| `st T x = e`    | Declares the variable `x` of type `T` with the spacetime `st`. |
| `when e then p else q end` | Executes `p` if `e` is equal to `true`, otherwise `q`. |
| `x <- e`        | Joins the information of `e` into `x`. |
| `pause`         | Delays the execution to the next instant. |
| `pause up`      | Delays and gives the control back to the upper universe. |
| `stop`          | Delays and gives the control back to the host language. |
| `par p`\|\|`q end`| Executes concurrently `p` and `q`. Terminates when both terminate. |
| `par p <> q end`| Same as \|\| but terminates in the next instant if one process terminates before the other (weak preemption). |
| `p ; q`         | Executes `p`, when it terminates, executes `q`. |
| `loop p end`    | Executes indefinitely `p`. When `p` terminates it is executed from its beginning again. |
| `abort when e in p end`| Executes `p` unless `e` is `true`, in which case the statement terminates in the current instant. |
| `suspend when e in p end`| Executes `p` in every instant in which `e` is `false`. |
| `universe with x in p end`| Executes `p` in a universe with the queue `x`. |

#