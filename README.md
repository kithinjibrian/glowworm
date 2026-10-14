# glowworm

A NumPy-style array library written in [Lugha](https://github.com/kithinjibrian/lugha).

Library code lives in `src/` (`source = "src"` in `lugha.toml`), so `src/array.la` is `glowworm.array`. Programs in `example/` import the library by name, with no manifest of their own:

```sh
lughac run example/main.la
```

Arrays are generic over their element type, print like NumPy, and use the usual operators element-wise:

```
import glowworm.array;

let a = array.from([2, 3], [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
let b: array.NdArray<f64> = array.ones([2, 3]);
println(a + b);                 // array([[2.0, 3.0, 4.0], [5.0, 6.0, 7.0]])
println(a.transpose());         // array([[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]])
println(a.mean());              // 3.5
```

| Area | API |
| --- | --- |
| Making | `from(shape, data)`, `full(shape, v)`, `zeros`, `ones`, `arange(n)`, `linspace(start, stop, n)` |
| Shape | `ndim()`, `count()`, `reshape(shape)`, `flatten()`, `transpose()` (2-D) |
| Indexing | `a.get([i, j])`, `a.set([i, j], v)`, `a.swap_rows(i, j)` (in place) |
| Arithmetic (`T: Num`) | `+ - * /` on equal shapes, `-a`, `scale(k)`, `abs()` |
| Reductions (`T: Num`) | `sum()`, `prod()`, `min()`, `max()`, `argmin()`, `argmax()` |
| Floats (`T: Float`) | `mean()`, `sqrt()`, `exp()` |
| Any type | `map(f)`, printing (`println(a)`) |

Linear algebra is in `glowworm.linalg`:

```
import glowworm.array;
import glowworm.linalg;

let a = array.from([2, 2], [4.0, 7.0, 2.0, 6.0]);
println(linalg.matmul(a, linalg.inv(a)));                  // the identity, up to rounding
println(linalg.solve(a, array.from([2], [18.0, 14.0])));   // array([1.0, 2.0])
```

| Function | What it does |
| --- | --- |
| `identity(n)` | the n × n identity (`T: Num`) |
| `dot(a, b)`, `outer(a, b)` | vector products |
| `matmul(a, b)` | matrix · matrix, or matrix · vector |
| `trace(a)` | sum of the diagonal |
| `norm(a)`, `det(a)`, `inv(a)`, `solve(a, b)` | for float matrices (`T: Float`); `inv` and `solve` panic on a singular matrix |

Run `lughac run example/linalg.la` to see them all.

`zeros`, `ones` and `arange` can't infer the element type alone: write `let n: array.NdArray<i64> = array.arange(5);`. Mismatched shapes, out-of-range indexes and empty reductions panic with a message.

To use it from another Lugha program, list it in that program's `lugha.toml`:

```toml
[packages]
glowworm = { git = "https://github.com/kithinjibrian/glowworm.git", version = "0.1" }
```

and `import glowworm.array;`.
