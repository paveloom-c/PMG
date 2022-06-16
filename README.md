### To reproduce

1. Check out this repository.

2. Make sure you have the following installed:

- [Rust](https://www.rust-lang.org)
- [Julia](https://julialang.org)
- [TexLive](https://tug.org/texlive)

3. Instantiate the project

   ```bash
   cargo build -r
   julia --project=. -e "using Pkg; Pkg.instantiate()"
   ```

4. Convert the data from the equatorial coordinate system
   to the Galactic heliocentric Cartesian system

   ```bash
   cargo run -r
   ```

5. Plot the projections in each plane:

   ```bash
   julia --project=. scripts/coords.jl
   ```

   *or*

   ```bash
   ./julia.bash scripts/coords.jl
   ```

### Notices

#### Mirrors

Repository:
- [Codeberg](https://codeberg.org/paveloom-c/PMG)
- [GitHub](https://github.com/paveloom-c/PMG)
- [GitLab](https://gitlab.com/paveloom-g/complex/PMG)
- [Radicle](https://app.radicle.network/seeds/pine.radicle.garden/rad:git:hnrkfwgg3khhx8keec53drptixg16xqhud3oo)

Reference:
- [GitHub Pages](https://paveloom-c.github.io/PMG)
- [GitLab Pages](https://paveloom-g.gitlab.io/complex/PMG)

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
