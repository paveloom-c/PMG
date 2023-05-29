### To get the results

1. Check out this repository.

2. Make sure you have installed the following:

    - [Nix](https://nixos.org)
    - [`direnv`](https://github.com/direnv/direnv)
    - [`nix-direnv`](https://github.com/nix-community/nix-direnv)

3. Allow `direnv` to load the environment by executing `direnv allow`.

4. While in the environment, run the `compute.bash` script.

Alternatively, you can download the results [here](https://drive.google.com/drive/folders/1L1lBVWZ8CMjjaifdMDBWdDOBWnpum_8D?usp=sharing) or [here](https://drive.google.com/drive/folders/1JsSwkPIN456PfPQSLpdbH9Gq_FcpahEo?usp=sharing).

### Notices

#### Mirrors

Repository:
- [Codeberg](https://codeberg.org/paveloom-c/PMG)
- [GitHub](https://github.com/paveloom-c/PMG)
- [GitLab](https://gitlab.com/paveloom-g/complex/PMG)

Reference:
- [GitHub Pages](https://paveloom-c.github.io/PMG)

#### Rust

This project provides [Rust](https://www.rust-lang.org) crates.
To build them, use [Cargo](https://doc.rust-lang.org/cargo).

#### Tests

To run tests, consider using [`nextest`](https://nexte.st).

#### KaTeX

To build a crate's documentation with [KaTeX](https://katex.org) support, run:

```bash
cargo doc
RUSTDOCFLAGS="--html-in-header assets/katex-header.html" cargo doc --no-deps --open
```

#### Julia

This project provides [Julia](https://julialang.org) scripts. Make sure to use
the project files (`Project.toml`) when running them:

```bash
julia --project=. -e "using Pkg; Pkg.instantiate()"
julia --project=. scripts/script.jl
```

Alternatively, you can use the `julia.bash` script, which starts a
[daemon](https://github.com/dmolina/DaemonMode.jl) and runs scripts through it:

```bash
julia --project=. -e "using Pkg; Pkg.instantiate()"
./julia.bash scripts/script.jl
```

To kill the daemon run

```bash
./julia.bash kill
```
