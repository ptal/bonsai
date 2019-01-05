# bonsai

<!--- [![ptal on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/ptal/bonsai.png
[travis]: https://travis-ci.org/ptal/bonsai
-->

Spacetime programming is a programming language on top of Java to describe search strategy when exploring a state-space, for example in constraint satisfaction problems.
Bonsai is the name of the compiler and runtime of this language.
Please consult the [Spacetime manual](http://hyc.io/spacetime) for more information.

## Build the manual

You might want to build the manual from the repository because you need it to be synchronized with a specific version of Bonsai or simply for offline usage.
Download the utility [mdbook](https://rust-lang-nursery.github.io/mdBook/):

```
cargo install mdbook
```

Once installed, go inside `bonsai/doc` and execute `mdbook build -o`.
The manual is generated inside a local folder named `book` and is directly opened in your browser.
