# sota staircase SideStepper

a fast .gitignore-respecting large file finder for .git repositories trying to
weed out large LFS files

**this is brain made software**: large language-based code generation has not
directly used here. but i'd be lying if i said i didn't ask chatgpt if there
was a better way to check a boolean result lol

## quickstart

### installing a binary

**note:** all non-windows builds are statically linked

- Windows
- Linux
- macOS universal
- macOS amd64
- macOS aarch64

(also available in the 'releases' tab wherever this repository is situated in)

### build it yourself

1. [get rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html#install-rust-and-cargo)
2. `cargo build release`

**nix users, rejoice:** `nix run github:markjoshwel/sidestepper` or `nix run git+:https://forge.joshwel.co/mark/sidestepper`

### running it

```text
./sidestepper
```

or on windows,

```text
./sidestepper.exe
```

it'll find for a `.git` directory in the current or parent directories, if you
want to use this not in the context i usually use this for, pass in
`--search-here` to treat the current working directory as the 'repository root'

it'll then make a `.sotaignore` file that i use in my other tooling,
but if you want output more friendly for integration in other places,
pass in `--plumbing` for it to output encountered large files, line-by-line, to
stdout

## historical changes

- v5 (i3/a4) - rewritten in rust lol
- v4 (i2/a4) - optimised single iod-ttt
- v3 (i2/a3) - faster matching by remembering ignored directories (ignore on demand, 'iod')
- v2 (i2/a2) - corrected ignored directory matching (named 'trytrytry')
- v1 (i1/a1) - original python script, still embedded within ReStepper

## licence

with all my heart, copyright (c) 2025 mark joshwel

the sota staircase SideStepper is permissively licenced, not needing
attribution, under the [0BSD licence](LICENCE). go ham.
