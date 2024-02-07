# stacklover
[![CI](https://github.com/nwtgck/stacklover-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/nwtgck/stacklover-rust/actions/workflows/ci.yml)

Zero-cost type for stack without complicated type or Box

## Why?

Rust requires concrete types in struct fields. Here is an example that creates an iterator and puts it into a struct. Its type is super super long and hard to maintain since the type is changed by its creation logic. Also, some types like closures can not be written out.

```rust
use std::convert::identity;
use std::sync::{Arc, Mutex};

// Super long and complicated type!!
type IteratorI32 = core::iter::Chain<
    core::iter::TakeWhile<
        core::iter::Map<core::ops::RangeFrom<i32>, fn(i32) -> i32>,
        fn(&i32) -> bool,
    >,
    core::iter::FlatMap<
        core::iter::Chain<
            core::iter::Map<core::str::Chars<'static>, fn(char) -> i32>,
            core::array::IntoIter<i32, 1>,
        >,
        [i32; 2],
        fn(i32) -> [i32; 2],
    >,
>;
// (Here is the end of type defintion!)

struct MyService {
    iter: Mutex<Option<IteratorI32>>,
}

impl MyService {
    fn put_iter(&self, message: &str) {
        // Create an iterator
        let iter: IteratorI32 = (1..)
            .map(identity::<fn(i32) -> i32>(|x| x * 3))             // NOTE: An extra identity function needed. Without this, an error "expected fn pointer, found closure" occurrs.
            .take_while(identity::<fn(&i32) -> bool>(|x| *x < 10))  // NOTE: An extra identity function needed.
            .chain(
                "HELLO"
                    .chars()
                    .map(identity::<fn(char) -> i32>(|c| c as i32))              // NOTE: An extra identity function needed.
                    .chain([message.len() as i32])
                    .flat_map(identity::<fn(i32) -> [i32; 2]>(|i| [i, i - 65])), // NOTE: An extra identity function needed.
            );
        // Put the iterator
        self.iter.lock().unwrap().replace(iter);
    }
    
    // ...
}
```

A simple solution is to use `Box` as shown below and allocate the iterator on the heap. 

```rust
use std::sync::{Arc, Mutex};

type IteratorI32 = Box<dyn Iterator<Item = i32> + Send + 'static>;

struct MyService {
    iter: Mutex<Option<IteratorI32>>,
}

impl MyService {
    fn put_iter(&self, message: &str) {
        // Create an iterator
        let iter: IteratorI32 = Box::new(
            (1..).map(|x| x * 3).take_while(|x| *x < 10).chain( // no extra identity function needed at all
                "HELLO"
                    .chars()
                    .map(|c| c as i32)
                    .chain([message.len() as i32])
                    .flat_map(|i| [i, i - 65]),
            ),
        );
        // Put the iterator
        self.iter.lock().unwrap().replace(iter);
    }
    
    // ...
}
```

However, we sometimes avoid unnecessary heap allocations and prefer stack allocations. Stack allocation may allow us to inlining functions or static dispatches and improve performance.

In order to avoid unnecessary `Box` and complicated type, `stacklover::define_struct!` macro is created. You can write the same logic without `Box` or complicated type as below.

```rust
use std::sync::{Arc, Mutex};

stacklover::define_struct! {
    IteratorI32,
    fn (message: &str) -> impl Iterator<Item = i32> {
        (1..).map(|x| x * 3).take_while(|x| *x < 10).chain(
            "HELLO"
                .chars()
                .map(|c| c as i32)
                .chain([message.len() as i32])
                .flat_map(|i| [i, i - 65]),
        )
    },
    impls = (Send, Sync),
}

struct MyService {
    iter: Mutex<Option<IteratorI32>>,
}

impl MyService {
    fn put_iter(&self, message: &str) {
        // Create an iterator
        let iter: IteratorI32 = IteratorI32::new(message);
        // Put the iterator
        self.iter.lock().unwrap().replace(iter);
    }
    
    // ...
}
```

stacklover also works with no_std.

## Performance

Performance with `stacklover` is the same as with bare type.

* bare: `impl Iterator<Item = i64>`
* boxed: `Box<impl Iterator<Item = i64>>`
* stacklover

Here is a result of [iter_benchmark](bench/benches/iter_benchmark.rs) in [GitHub Actions](https://github.com/nwtgck/stacklover-rust/actions/workflows/ci.yml).

```txt
iterator sum/bare       time:   [12.329 ns 12.487 ns 12.670 ns]
iterator sum/boxed      time:   [348.60 ms 352.41 ms 356.41 ms]
iterator sum/stacklover time:   [11.494 ns 11.657 ns 11.819 ns]
```

Note that the time unit of "boxed" is ms not ns. Investigation is underway to know why Box is too slow.

Data size of struct created by `stacklover` is the same as bare type:
https://github.com/nwtgck/stacklover-rust/blob/796c2dcff68032dbbc064314018565cb21d8c94f/tests/lib.rs#L19


## How to use

Add `stacklover` to the `Cargo.toml`.

###  Dependency

```toml
[dependencies]
stacklover = { git = "https://github.com/nwtgck/stacklover-rust.git", rev = "efbe91996e5554a9bf7bf3c35bd67c4ed2770866" }
```

### Simple example
Create by `YourStruct::new()` and access its inner value by `.as_ref()`, `.as_mut()`, `.into_inner()` and `.as_pin_mut()`. Here is an example.

```rust
use std::fmt::Debug;

fn main() {
    // Use `define_struct!` everywhere `struct YourStruct;` can be used.
    stacklover::define_struct! {
        // Struct name to be defined.
        IteratorI32,
        // Note that the function below has no name.
        fn (i: i32) -> impl Iterator<Item = i32> + Debug {
            (1..i).map(|x| x * 3).take_while(|x| *x < 10)
        },
        impls = (Send, Sync, Debug),
    }

    // Use `IteratorI32` instead of complicated type.
    let mut x: IteratorI32 = IteratorI32::new(10);

    println!("x={:?}", x);
    // Output:
    // x=TakeWhile { iter: Map { iter: 1..10 }, flag: false }

    println!("size_hint={:?}", x.as_ref().size_hint());
    // Output:
    // size_hint=(0, Some(9))

    println!("next={:?}", x.as_mut().next());
    // Output:
    // next=Some(3)

    // Get `Iterator<Item = i32>` by .into_inner()
    let iter /* : impl Iterator<Item = i32> */ = x.into_inner();
    for i in iter {
        println!("i={}", i)
    }
    // Output:
    // i=6
    // i=9
}
```

### Async function example
Async function can be used as well.

```rust
#[tokio::main]
async fn main() {
    stacklover::define_struct! {
        IteratorI32,
        // async function
        async fn (i: i32) -> impl Iterator<Item = i32> {
            (1..i).map(|x| x * 3).take_while(|x| *x < 10)
        },
        impls = (Send, Sync),
    }

    let x: IteratorI32 = IteratorI32::new(10).await; // .await used
    let iter /* : impl Iterator<Item = i32> */ = x.into_inner();
    for i in iter {
        println!("i={}", i)
    }
    // Output:
    // i=3
    // i=6
    // i=9
}
```

### Wrapped type example
You can store the wrapped type `T` in `Result<T>`.  Here is an example using a creation function that returns `Result<impl Iterator<...>>`, but the struct `IteratorI32` stores `impl Iterator<...>`.

```rust
stacklover::define_struct! {
    IteratorI32,
    // The function below returns Result but the inner value should be Iterator, not Result.
    fn (i: i32) -> Result<impl Iterator<Item = i32>, std::io::Error> {
        let iter = (1..i).map(|x| x * 3).take_while(|x| *x < 10);
        Ok(iter)
    },
    // Specify parameters in the following order.
    impls = (Send, Sync),
    inner_type = impl Iterator<Item=i32>,
    wrapped_type = Result<__Inner__, std::io::Error>, // Type paramter `__Inner__` is predefined in the macro.
    to_wrapped_struct = |result, inner_to_struct| { result.map(inner_to_struct) },
}

// Use `IteratorI32` instead of complicated type.
let result: Result<IteratorI32, std::io::Error> = IteratorI32::new(10);
// Get IteratorI32 from Result by ?.
let x: IteratorI32 = result?;
```

Here is an example with more nested type and async function.

```rust
stacklover::define_struct! {
    IteratorI32,
    async fn (i: i32) -> Result<(impl Iterator<Item = i32>, String, f32), std::io::Error> {
        let iter = (1..i).map(|x| x * 3).take_while(|x| *x < 10);
        Ok((iter, "hello".to_owned(), 3.14))
    },
    // Specify parameters in the following order.
    impls = (Send, Sync),
    inner_type = impl Iterator<Item=i32>,
    wrapped_type = Result<(__Inner__, String, f32), std::io::Error>, // Type paramter `__Inner__` is predefined in the macro.
    to_wrapped_struct = |result, inner_to_struct| { result.map(|(iter, s, f)| (inner_to_struct(iter), s, f)) },
}

let result: Result<(IteratorI32, String, f32), std::io::Error> = IteratorI32::new(10).await;
// Get a tuple with IteratorI32 from Result by ?.
let (iter, s, f): (IteratorI32, String, f32) = result?;
```

### Implement trait example

In `impls = (...)` you can specify the following traits.
* [Send](https://doc.rust-lang.org/std/marker/trait.Send.html)
* [Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)
* [Unpin](https://doc.rust-lang.org/std/marker/trait.Unpin.html)
* [UnwindSafe](https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html)
* [RefUnwindSafe](https://doc.rust-lang.org/std/panic/trait.RefUnwindSafe.html)
* [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
* [Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)
* [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
* [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
* [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)
* [Copy](https://doc.rust-lang.org/core/marker/trait.Copy.html)
* [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html)
* [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)

At compile time, it is asserted whether the inner type implements the trait in `impls = (...)`. 

### Using attributes - auto_enums
Attributes can be used. Here is an example using [`auto_enums`](https://github.com/taiki-e/auto_enums), which allows us to return different types without heap allocations.

```rust
fn main() {
    stacklover::define_struct! {
        AutoEnumIterator,
        #[auto_enums::auto_enum(Iterator)]
        fn (x: i32) -> impl Iterator<Item=i32> {
            match x {
                0 => 1..10,
                _ => vec![5, 10].into_iter(),
            }
        },
        impls = (),
    }

    let x1: AutoEnumIterator = AutoEnumIterator::new(0);
    let iter1 /* : impl Iterator<Item = i32> */ = x1.into_inner();
    println!("iter1={:?}", iter1.collect::<Vec<_>>());
    // Output: iter1=[1, 2, 3, 4, 5, 6, 7, 8, 9]

    let x2: AutoEnumIterator = AutoEnumIterator::new(4);
    let iter2 /* : impl Iterator<Item = i32> */ = x2.into_inner();
    println!("iter2={:?}", iter2.collect::<Vec<_>>());
    // Output: iter2=[5, 10]
}
```

### Define separately for IDE and formatter
You can define the creation function separately. This may allow you to have better IDE and formatter support. 
```rust
stacklover::define_struct! {
    IteratorI32,
    fn (message: &str) -> impl Iterator<Item = i32> {
        // Call the function
        create_iter_i32(message)
    },
    impls = (),
}

// Define a creation function separately
fn create_iter_i32(message: &str) -> impl Iterator<Item = i32> {
    (1..).map(|x| x * 3).take_while(|x| *x < 10).chain(
        "HELLO"
            .chars()
            .map(|c| c as i32)
            .chain([message.len() as i32])
            .flat_map(|i| [i, i - 65]),
    )
}
```

## How it works

Read [example1.expanded.rs](expand/src/bin/example1.expanded.rs) expanded from [example1.rs](expand/src/bin/example1.rs).
