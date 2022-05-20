### To reproduce

1. Check out this repository.

2. Make sure you have the following installed:

- [Julia](https://julialang.org)
- [TexLive](https://tug.org/texlive)

3. Instantiate the project

    ```bash
    julia --project=. -e "using Pkg; Pkg.instantiate()"
    ```

4. Convert the data to the Galactic heliocentric Cartesian system and plot the projections in each plane:

   ```bash
   julia --project=. scripts/xyz.jl
   ```

   *or*

   ```bash
   ./julia.bash scripts/xyz.jl
   ```

### Notices

#### Mirrors

Repository:
- [Codeberg](https://codeberg.org/paveloom-c/PMG)
- [GitHub](https://github.com/paveloom-c/PMG)
- [GitLab](https://gitlab.com/paveloom-g/complex/PMG)
- [Radicle](https://app.radicle.network/seeds/pine.radicle.garden/rad:git:hnrkfwgg3khhx8keec53drptixg16xqhud3oo)

#### Julia

This project provides [Julia](https://julialang.org) scripts. Make sure to use the project files (`Project.toml`) when running them:

```bash
julia --project=. -e "using Pkg; Pkg.instantiate()"
julia --project=. scripts/script.jl
```

Alternatively, you can use the `julia.bash` script, which starts a [daemon](https://github.com/dmolina/DaemonMode.jl) and runs scripts through it:

```bash
julia --project=. -e "using Pkg; Pkg.instantiate()"
./julia.bash scripts/script.jl
```

To kill the daemon run

```bash
./julia.bash kill
```
