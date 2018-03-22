# `copy_from_str`

[![Build Status](https://travis-ci.org/xfix/copy_from_str.svg?branch=master)](https://travis-ci.org/xfix/copy_from_str)

Extension methods for copying strings and characters into a string.

This crate provides `copy_from_str` function which can be used to
mutate Rust strings. It works similarly to [`copy_from_slice`] from
standard library except it is for strings.

# Examples

```rust
use copy_from_str::CopyFromStrExt;

fn make_ascii_uppercase(mut input: &mut str) {
    let mut buffer = [0; 4];
    while let Some(ch) = input.chars().next() {
        let end_position = ch.len_utf8();
        let src = ch.to_ascii_uppercase().encode_utf8(&mut buffer);
        input[..end_position].copy_from_str(src);
        input = &mut {input}[end_position..];
    }
}

let mut str = String::from("Hello, world! 💯");
make_ascii_uppercase(&mut str);
assert_eq!(str, "HELLO, WORLD! 💯");
```

[`copy_from_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice
