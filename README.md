# glowworm

A NumPy-style array library written in [Lugha](https://github.com/kithinjibrian/lugha).

Library code lives in `src/` (`source = "src"` in `lugha.toml`), so `src/array.la` is `glowworm.array`. Programs in `example/` import the library by name, with no manifest of their own:

```sh
lughac run example/main.la
```

Arrays are generic over any number type and use the usual operators, element-wise:

```
import glowworm.array;

let a = array.from([2, 3], [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
let b: array.NdArray<f64> = array.ones([2, 3]);
let c = (a + b) * a - b.scale(2.0);    // shapes must match
println(c.sum());
```

To use it from another Lugha program, list it in that program's `lugha.toml`:

```toml
[packages]
glowworm = { git = "https://github.com/kithinjibrian/glowworm.git", version = "0.1" }
```

and `import glowworm.array;`.
