# Contributing to RustQuant

If you're reading this, I assume you're interested in contributing to this project. 
If so, thank you very much! I would love to grow the project and have a solid, 
stable library by the end of 2023.

Firstly, this contribution guide is a work in progress, so bare with me. 

I welcome contributions of all kinds, including:

* Bug reports.
* Bug fixes.
* Feature requests.
* Feature implementations.
* Documentation improvements.
* Unit tests.

If you have any ideas, feel free to make an issue to discuss it, 
or just make a pull request.

Additionally, feel free to contact me directly at: rustquantcontact@gmail.com

If you decide to contribute, please include the following 
license header in any files you create:

```rust
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE.md or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

Additionally, all code should be formatted with `rustfmt`, and all documentation 
should be formatted with `rustdoc`, and all code should be tested 
as thoroughly as possible (hopefully keeping coverage >80%).

I like to separate my code as below, as I think it improves the readability
a lot, and I would encourage any PRs to do the same (or similar). Of course, 
any suggestions or opinions on different styles are welcome!

```rust
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Structs, enums, and traits
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

enum Enum {}

struct Struct {}

trait Trait {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Implementations, functions, and macros
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Struct {}

impl Trait for Struct {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn very_thorough_test() {}
}
```

Thank you for your interest in contributing to RustQuant!