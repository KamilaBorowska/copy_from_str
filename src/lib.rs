//! Extension methods for copying strings into a string.
//!
//! This crate provides [`copy_from_str`] function which can be used to
//! mutate Rust strings. It works similarly to [`copy_from_slice`] from
//! standard library except it is for strings.
//!
//! # Examples
//!
//! ```
//! use copy_from_str::CopyFromStrExt;
//!
//! fn make_ascii_uppercase(mut input: &mut str) {
//!     let mut buffer = [0; 4];
//!     while let Some(ch) = input.chars().next() {
//!         let src = ch.to_ascii_uppercase().encode_utf8(&mut buffer);
//!         let (to_uppercase, rest) = { input }.split_at_mut(ch.len_utf8());
//!         to_uppercase.copy_from_str(src);
//!         input = rest;
//!     }
//! }
//!
//! let mut str = String::from("Hello, world! 💯");
//! make_ascii_uppercase(&mut str);
//! assert_eq!(str, "HELLO, WORLD! 💯");
//! ```
//!
//! [`copy_from_str`]: trait.CopyFromStrExt.html#tymethod.copy_from_str
//! [`copy_from_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice

#![deny(missing_docs)]
#![no_std]

/// Extension method for copying a string into another string.
pub trait CopyFromStrExt {
    /// Copies all elements from `src` into `self`, using a memcpy.
    ///
    /// The length of `src` must be the same as `self`.
    ///
    /// # Panics
    ///
    /// This function will panic if the two strings have different lengths.
    ///
    /// # Examples
    ///
    /// Copying two elements from a string into another:
    ///
    /// ```
    /// use copy_from_str::CopyFromStrExt;
    ///
    /// let src = "abcd";
    /// let mut dst = String::from("  ");
    ///
    /// dst.copy_from_str(&src[2..]);
    ///
    /// assert_eq!(src, "abcd");
    /// assert_eq!(dst, "cd");
    /// ```
    ///
    /// Rust enforces that there can only be one mutable reference with no
    /// immutable references to a particular piece of data in a particular
    /// scope. Because of this, attempting to use `copy_from_str` on a
    /// single string will result in a compile failure:
    ///
    /// ```compile_fail
    /// use copy_from_str::CopyFromStrExt;
    ///
    /// let mut string = String::from("abcde");
    ///
    /// string[..2].copy_from_str(&string[3..]); // compile fail!
    /// ```
    ///
    /// To work around this, we can use [`split_at_mut`] to create two distinct
    /// sub-strings from a string:
    ///
    /// ```
    /// use copy_from_str::CopyFromStrExt;
    ///
    /// let mut string = String::from("abcde");
    ///
    /// {
    ///     let (left, right) = string.split_at_mut(2);
    ///     left.copy_from_str(&right[1..]);
    /// }
    ///
    /// assert_eq!(string, "decde");
    /// ```
    ///
    /// [`split_at_mut`]: https://doc.rust-lang.org/std/primitive.str.html#method.split_at_mut
    #[allow(unused_attributes)]
    #[inline]
    fn copy_from_str(&mut self, src: &str);
}

impl CopyFromStrExt for str {
    #[inline]
    fn copy_from_str(&mut self, src: &str) {
        unsafe { self.as_bytes_mut() }.copy_from_slice(src.as_bytes());
    }
}
