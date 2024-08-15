# Contributing to `sealed`

- Ensure you're using Rust 1.61.0 to develop and test.
  This is required because we test macro error messages, which are not stable across
  compiler versions. Pinning the version everyone uses when developing `sealed` was the
  solution I found. If you know of a better solution,
  [let me know](https://github.com/jmg-duarte/sealed-rs/issues/new)!

- If you're adding new functionality, add tests to the new functionality and existing
  ones, ensuring they all play nice with each other!

- If you're fixing existing functionality, start by adding a minimal reproducible
  example as a test and fix the code until you achieve the desired behavior.

Have fun!
