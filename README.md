# bonsai

<!--- [![ptal on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/ptal/bonsai.png
[travis]: https://travis-ci.org/ptal/bonsai
-->

[Companion guide (PPDP19)](http://hyc.io/spacetime/PPDP19.html) (see below if the link is dead).

Spacetime programming is a programming language on top of Java to describe search strategies exploring combinatorial state-space such as in constraint satisfaction problems.
Please consult the [spacetime manual](http://hyc.io/spacetime) for more information.

## Build the manual

You might want to build the manual from the repository because you need it to be synchronized with a specific version of Bonsai or simply for offline usage.
Download the utility [mdbook](https://rust-lang-nursery.github.io/mdBook/):

```
cargo install mdbook
```

Once installed, go inside `bonsai/doc` and execute `mdbook build -o`.
The manual is generated inside a local folder named `book` and is directly opened in your browser.
