# LearningRust
[Cargo](https://doc.bccnsoft.com/docs/rust-1.36.0-docs-html/cargo/guide/creating-a-new-project.html)

## Table of Contents
1. [Cargo](#cargo)

## Cargo
Cargo is a packaging tool used for Rust. It ensures repeatable builds.

* `cargo new <pkg name> <mode>`
    * `<mode>` can be `--lib` or `--bin` (has `main()` entrypoint)
    * Creates `*.toml` manifest with metadata about package.
* `cargo build`
    * Compiles code.
    * Run w/ command `./target/debug/<pkg name>`
    * `--debug` 
        * Default option.
        * **Slower to run but compiles faster**
        * Doesn't go through optimizations.
    * `--release`
        * **Faster to run but compiles slower.**
        * Undergoes optimizations.
* `cargo run`
    * Compiles and runs code.
    * Both `cargo build/run` generate a `.lock` file with dependecies listed.

### Dependecies
* Listed in `.toml` under `[Dependencies]` section.
* To **add a dependency**:
    1. Add name and version of dependency.
    2. Run `cargo build` to fetch that pkg.
* `Cargo.toml` vs. `Cargo.lock`
    * `toml` is written by you and is broad.
    * `lock` is maintained by Cargo and shouldn't be manually edited. Is exact.