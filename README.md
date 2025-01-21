<br/><br/>
<div>
    <h3 align="center">🦀 Crucible</h3>
    <p align="center">
      A simple compiler written to learn about intermediate representation and optimization techniques.
    </p>
</div>
<br><br>

This project is purely for experimenting while I was researching compiler optimization techniques.

```shell
cargo run
```

1. The source is lexed to get tokens.
2. Next the parser creates an abstract syntax tree.
3. The AST is lowered into a Static Single Assignment format (SSA) as intermediate representation (IR).
4. The IR is analyzed for optimizations. Optimizations implemented: dead code elimination, constant folding.

Example source:
```ts
let x: int = 3;
let unused: int = 0;
let y: int = x + 1;
let z: int = x * y / 2;
z = z + 1;
```

Initial intermediate representation:
```
[
    Constant { result: "x.1", value: 3 },
    Constant { result: "unused.1", value: 0 },
    Binary { result: "y.1", op: Add, left: "x.1", right: "1" },
    Binary { result: "bin.1", op: Multiply, left: "x.1", right: "y.1" },
    Binary { result: "z.1", op: Divide, left: "bin.1", right: "2" },
    Binary { result: "z.2", op: Add, left: "z.1", right: "1" }
]
```

Dead code eliminated (remove code that can't affect the program):
```
[
    Constant { result: "x.1", value: 3 },
    Binary { result: "y.1", op: Add, left: "x.1", right: "1" },
    Binary { result: "bin.1", op: Multiply, left: "x.1", right: "y.1" },
    Binary { result: "z.1", op: Divide, left: "bin.1", right: "2" }
]
```

Constant folding (identify and evaluate constant expressions at compile time):
```
[
    Constant { result: "x.1", value: 3 },
    Constant { result: "y.1", value: 4 },
    Constant { result: "bin.1", value: 12 },
    Constant { result: "z.1", value: 6 }
]
```
