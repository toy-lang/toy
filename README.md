<div align="center">
    <a href="https://crates.io/crates/toylang"><img alt="Crates.io" src="https://img.shields.io/crates/v/toylang?style=flat-square">
        <img alt="Crates.io" src="https://img.shields.io/crates/d/toylang?style=flat-square"></a>
    <img alt="Lines of code" src="https://img.shields.io/tokei/lines/github/toy-lang/toy?style=flat-square">
</div>

# Toylang
Toylang is a (WIP) trait (also known as an "interface") oriented programming language developed by Teamminty, aiming to be similar to rust, but fully trait oriented and with insanely fast compile times with multithreading and cranelift, one of the fastest compiler backends ever. With compile times that fast, it feels like an interpreted lamguage! Toylang also aims to have some of rust's safety guarentees. We have optional lifetimes, if you don't use them we have the compile find the right time to drop the data..

## Install
Installation instructions can be found [here](https://example.com).

## Usage
### Windows:
1. Set the default app for `.toyc` files to toyvm (Should be set automatically in future)
2. Double click the `.toyc` file to run it

### Unix (MacOS/Linux)
```sh
toyvm path/to/toyc/file
```

## Usage (For developers)
<!-- TODO -->
