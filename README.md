<div align="center">
  <h1><code>component-opt</code></h1>

  <p>
    <strong>An optimizer for Wasm Components</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/component-opt"><img src="https://img.shields.io/crates/v/component-opt.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/component-opt"><img src="https://img.shields.io/crates/d/component-opt.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/component-opt"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
  
  <p>
    <a href="https://techforpalestine.org/learn-more"><img src="https://badge.techforpalestine.org/default" alt="build status" /></a>
  </p>
</div>

# Current Status

This project currently only offers one optimization and does not allow it to be configured.

That optimization is simply to unpack the modules within the component and optimize them.
It makes extensive use of 
* `wasmparser` for parsing components,
* `wasm-opt` for performing module optimization, and
* `wasm-encoder` for constructing th output component.

The current testing methodology is explained in the [Tests README](./tests/README.md)

# Goals

In the long run, I'd like this project to evolve to include "Component-aware" optimizations.

I've created a [Optimization Pass Issue Tracker](https://github.com/esoterra/component-opt/issues/1) for proposed optimization passes.

