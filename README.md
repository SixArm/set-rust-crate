# set! macro Rust crate

This crate provides `set!` macros to create set collections and
insert elements. This is inspired by the `vec!` macro.

## set! macro

Create a new set collection and insert elements.

Example with tuple syntax:

```rust
let m = set!((1, 2), (3, 4));
```

Example with arrow syntax:

```rust
let m = set!(1 => 2, 3 => 4);
```

Equivalent Rust standard code:

```rust
let mut m = HashSet::new();
m.insert(1, 2);
m.insert(3, 4);
```

## set_insert! macro

Use an existing set collection and insert elements.

Example with tuple syntax:

```rust
let mut m = HashSet::new();
set_insert!(m, (1, 2), (3, 4));
```

Example with arrow syntax:

```rust
let mut m = HashSet::new();
set_insert!(m, 1 => 2, 3 => 4);
```

Equivalent Rust std code with method `insert``:

```rust
let mut m = HashSet::new();
m.insert(1, 2);
m.insert(3, 4);
```
