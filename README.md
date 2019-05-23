# types

Rust types conditions, wrap `std::any::Typed`.

### Examples

## ty\_cond
``` rust
assert_eq!(types::ty_cond(&0, &1), true);
assert_ne!(types::ty_cond(&0, &String::default()), true);

```

## Conds
```rust
use types::Conds;

// ints
assert_eq!(0_i8.is_i8(), true);
assert_eq!(0_i16.is_i16(), true);
assert_eq!(0_i32.is_i32(), true);
assert_eq!(0_i64.is_i64(), true);
assert_eq!(0_i128.is_i128(), true);

assert_ne!(0_u8.is_i8(), true);
assert_ne!(0_u16.is_i16(), true);
assert_ne!(0_u32.is_i32(), true);
assert_ne!(0_u64.is_i64(), true);
assert_ne!(0_u128.is_i128(), true);


// uints
assert_eq!(0_u8.is_u8(), true);
assert_eq!(0_u16.is_u16(), true);
assert_eq!(0_u32.is_u32(), true);
assert_eq!(0_u64.is_u64(), true);
assert_eq!(0_u128.is_u128(), true);

assert_ne!(0_i8.is_u8(), true);
assert_ne!(0_i16.is_u16(), true);
assert_ne!(0_i32.is_u32(), true);
assert_ne!(0_i64.is_u64(), true);
assert_ne!(0_i128.is_u128(), true);

// floats
assert_eq!(0_f32.is_f32(), true);
assert_eq!(0_f64.is_f64(), true);

assert_ne!(0_i32.is_f32(), true);
assert_ne!(0_i64.is_f64(), true);
```
