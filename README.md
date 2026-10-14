# glowworm

A NumPy-style array library written in [Lugha](https://github.com/kithinjibrian/lugha).

Library code lives in `src/` (`source = "src"` in `lugha.toml`), so `src/array.la` is `glowworm.array`. Programs in `example/` import the library by name, with no manifest of their own:

```sh
lughac run example/main.la
```

To use it from another Lugha program, list it in that program's `lugha.toml`:

```toml
[packages]
glowworm = { git = "https://github.com/kithinjibrian/glowworm.git", version = "0.1" }
```

and `import glowworm.array;`.
