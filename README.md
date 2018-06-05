# `copy_from_str`

Extension methods for copying strings into a string.

This crate provides `copy_from_str` function which can be used to
mutate Rust strings. It works similarly to [`copy_from_slice`] from
standard library except it is for strings.

# Examples

```rust
use copy_from_str::CopyFromStrExt;

fn make_ascii_uppercase(mut input: &mut str) {
    let mut buffer = [0; 4];
    while let Some(ch) = input.chars().next() {
        let src = ch.to_ascii_uppercase().encode_utf8(&mut buffer);
        let (to_uppercase, rest) = { input }.split_at_mut(ch.len_utf8());
        to_uppercase.copy_from_str(src);
        input = rest;
    }
}

let mut str = String::from("Hello, world! 💯");
make_ascii_uppercase(&mut str);
assert_eq!(str, "HELLO, WORLD! 💯");
```

[`copy_from_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice
