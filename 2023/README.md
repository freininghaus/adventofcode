# Advent of Code 2023 in Rust

## Learnings

### Day 1
*   Read the contents of a file into an `io::Result<String>` with [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html):

    ```rust
    fn input() -> String {
        std::fs::read_to_string("input/day01.txt")
            .expect("Could not read file")
    }
    ```
*   Iterate over the Unicode characters of a `&str` with [`.chars()`](https://doc.rust-lang.org/std/primitive.str.html#method.chars).
*   Convert a `char` to an optional decimal digit (`Option<u32>`) with [`.to_digit(10)`](https://doc.rust-lang.org/std/primitive.char.html#method.to_digit)
*   Collect an iterator to a `Vec` containing the items with `.collect::<Vec<_>>()`.
*   Convert a `&str` to a byte slice (`&[u8]`) with [`.as_bytes()`](https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes).

    This may be more convenient than working with a `&str` because we can easily build arbitrary sub-slices of bytes, but not of `&str` unless we guarantee that we never put a slice boundary into the middle of a Unicode character.
*   If we have an iterator with item type `Option<_>`, `.flat_map()` is useful for filtering out `None` values.
*   A slice of `Vec`s or strings can be concatenated with the [`.concat()`](https://doc.rust-lang.org/std/primitive.slice.html#method.concat) method.
*   `.iter()` iterates over references, `.into_iter()` iterates over values.
*   Slices have [`.starts_with()`](https://doc.rust-lang.org/std/primitive.slice.html#method.starts_with) and `.ends_with()`.
*   Constant string slices can be defined with `const` at module level:

    ```rust
        const TEST_INPUT_1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    ```
*   `&[u8]` literals can be built with `b"foo"`.
*    Split a string slice into lines with `.lines()`.