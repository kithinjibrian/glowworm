# glowworm

A NumPy-style array library written in [Lugha](https://github.com/kithinjibrian/lugha).

```sh
lughac run main.la
```

To use it from another Lugha program, list it in that program's `lugha.toml`:

```toml
[packages]
glowworm = { git = "https://github.com/kithinjibrian/glowworm.git", version = "0.1" }
```

and `import glowworm.array;`.
