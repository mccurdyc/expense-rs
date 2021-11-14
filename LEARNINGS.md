- modules don't implicitly include other files in the filesystem.
- json (de)serialization is specified via an annotation.
- https://stackoverflow.com/a/68795912
- Use `Option<...>` for fields that may not exist when JSON (de)serialization.
- https://serde.rs/lifetimes.html
- `String` is a mutable reference to a string on the heap?
- use after `move`

```rust
struct Foo {
  a: Option<i64>,
  b: Option<i64>,
}
...
let foo = Foo {
  a: 1,
  b: 2,
}

// This is fine
let a = foo.a;
let b = foo.b;

// ERROR - use after move on second line.
// let a = foo.a.unwrap();
// let b = foo.b.unwrap(); // ERROR
```

This is fine because a copy of foo happens. But, this breaks if in the `let a` line,
you call a method that takes ownership of `foo` e.g., `.unwrap()`.

TODOs
- Need to better understand the proper patterns of dealing with `Option` i.e., avoiding `unwrap()`.
  - is it `if let Some(foo)`? Or some use of `map`?
  - https://stackoverflow.com/questions/64996954/how-can-i-pull-data-out-of-an-option-for-independent-use
  - `unwrap()` and `expect()` take ownership of `self`.
  -AH!!! I bet I need to use a constructor function that takes ownership!
