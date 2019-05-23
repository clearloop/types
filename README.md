# types

Rust types conditions, wrap `std::any::Typed`.

## Examples

### ty\_cond
``` rust
assert_eq!(types::ty_cond(&0, &1), true);
assert_ne!(types::ty_cond(&0, &String::default()), true);

```

### Conds
```rust
use types::Conds;

// ints
assert_eq!(0_i8.is_i8(), true);
assert_ne!(0_u8.is_i8(), true);

// uints
assert_eq!(0_u8.is_u8(), true);
assert_ne!(0_i8.is_u8(), true);

// floats
assert_eq!(0_f32.is_f32(), true);
assert_ne!(0_i32.is_f32(), true);
```

## LICENSE
[MIT][1]

[1]: https://choosealicense.com/licenses/mit/
