# Exercise 2: Pattern Matching with `match`

In this exercise, you'll learn one of Rust's most powerful features: pattern matching with `match`. Think of it as JavaScript's `switch` statement, but with superpowers that prevent entire categories of bugs.

<div class="workshop-objectives">

### What you'll learn

- **Pattern matching**: Rust's `match` expression and how it compares to JavaScript's `switch`
- **Exhaustiveness checking**: Why Rust forces you to handle all cases (and why that's a good thing)
- **The `_` wildcard**: A catch-all pattern for everything else
- **String slices**: The difference between `&str` and `String`

</div>

## The Goal

We want to build a filter selector: a function that takes an image and a filter name, applies the matching filter, and returns the transformed image. Users will be able to request filters like `"cali"`, `"dramatic"`, or `"lofi"` by name.

Let's start by looking at the function signature in `exercises/src/exercise_2.rs`:

```rust
pub fn transform(img: &mut PhotonImage, filter: &str) {
    todo!()
}
```

This function takes two parameters:

- `img: &mut PhotonImage` — the input image as a mutable reference, so we can modify it
- `filter: &str` — the name of the filter to apply

But wait, what's that `&str` type? If you're coming from JavaScript, you might expect just `String`. Let's understand this first, because it's fundamental to how Rust works.

## String Slices: `&str` vs `String`

In JavaScript, and most other languages that use garbage collection, there's only one string type. You create strings, pass them around, and the runtime figures out when to free the memory. Rust takes a different approach — one that gives you more control and in-turn allows for higher performance code.

### Ownership: Who's Responsible for This Data?

In Rust, every value has exactly one **owner** — the variable responsible for that data. When that owner goes out of scope (like when a function returns), Rust automatically frees the memory. No garbage collector needed.

Think of it like checking out a library book. When you check out a book, you're responsible for it. You can read it, give it to a friend (but then they're responsible for it), or return it. The library (in this case the compiler) always knows exactly who has each book,
and unlike a real library can make sure borrowers _always_ return their books when they are done with them.

```rust
fn example() {
    let name = String::from("cali");  // `name` owns this string
    // ... use name ...
}  // `name` goes out of scope here, memory is automatically freed
```

we can also transfer ownership to someone else:

```rust,editable
fn main() {
    let name = String::from("cali");  // `name` owns this string
    
    other_function(name); // we give ownership of the name to `other_function`!
    
    // we cannot use `name` anymore because we gave it away!
    // uncomment the line below to see the compiler error
    // println!("{}", name);
}

fn other_function(name: String) {
    // ... use name ...
    // because we took ownership, `name` goes out of scope here, and memory is automatically freed
}
```

This is fundamentally different from JavaScript, where the garbage collector periodically scans memory to find unused data. Rust's approach has zero runtime overhead. Memory management is determined at compile time.

### References: Borrowing Without Taking Ownership

But what if you want to _use_ a value without taking ownership of it? That's where **references** come in. A reference lets you access data without becoming responsible for it (like reading someone else's library book).

In Rust, references are created with the `&` symbol and come in two flavors:

- `&T` — An **immutable** reference (read-only access)
- `&mut T` — A **mutable** reference (read and write access)

```rust,editable
fn main() {
    let name = String::from("cali");
    
    other_function(&name); // `other_function` borrows `name`, doesn't take ownership
    
    // we now **can** continue to use `name` because we still have ownership!
    // just just temporarily lend out access to it
    println!("{}", name);
}

fn other_function(name: &str) {
    // ... use name ...
    // as we only have a borrow of the string, it will *not* be freed when going out of scope
}
```

> Aside: Rust enforces one critical rule: at any given time, you can have _either_ multiple immutable references _or_ one mutable reference, but **never both**.
>
> If you attempt to violate this (by having multiple mutable references to the same piece of data) your program will not compile.
>
> ```rust,compile_fail
> let mut name = String::from("cali");
> let b = &name;
> update_name(&mut name);
> println!("{b}");
>
> fn update_name(name: &mut str) {
>   // ... modifies the name here ...
> }
> ```

Now we can understand the two string types:

- `String` — An **owned** string. The variable holding it is responsible for the memory and can modify its contents.
- `&str` — An **immutable reference** to string data. It borrows the string without owning it. It cannot manipulate the string, only read from it.

When you write a string literal like `"cali"` in your code, its type is `&str`. The text is _baked_ into your compiled program, and you get a reference to it.

### Why Our Function Uses `&str`

Let's look at our function signature again:

```rust,noplayground
pub fn transform(mut img: PhotonImage, filter: &str) -> PhotonImage
```

We use `&str` for the filter name because:

1. **We only need to read it** — We're just comparing the filter name to decide which filter to apply. We don't need to modify it or keep it around.

2. **It's more flexible** — A `&str` parameter can accept both string literals (`"cali"`) _and_ references to `String` values (`&my_string`). If we required `String`, callers would have to allocate a new string every time.

3. **It's more efficient** — Passing a reference is cheap (it's just a pointer). Passing an owned `String` would require either moving ownership or cloning the data.

As a rule of thumb: **use `&str` when you just need to read string data**. Only use `String` or `&mut str` when you absolutely know you need to own or modify the string.

### What about `&String`?

The keen reader might have noticed that there is actually a fourth combination we could have used, and that's `&String`. While it does technically exist, it's not very useful. It represents a borrow of an owned `String`, meaning the caller needs to own the allocation. But since we only have an **immutable** reference to it anyway, we still cannot manipulate the string, only read from it. Additionally, anything you can do with a `&String` you can also do with a `&str`, so it's the one used in practice.

## Introducing `match`

Rust's `match` is like JavaScript's `switch`, but safer. Here's our filter selector:

```rust,noplayground
pub fn transform(img: &mut PhotonImage, filter: &str) {
    match filter {
        "cali" => photon::filters::cali(img),
        "dramatic" => photon::filters::dramatic(mg),
        _ => panic!("no such filter"),
    }
}
```

The `_` is a **wildcard** that matches anything (like `default:` in JavaScript).

Unlike JavaScript's `switch`, there's no fall-through and no `break` needed — each arm is self-contained. More importantly, Rust **requires** you to handle every possible case. Try removing the `_ => panic!(...)` line and the compiler will reject your code:

```text
error[E0004]: non-exhaustive patterns: `&_` not covered
```

This might feel strict, but it catches bugs at compile time that JavaScript would only reveal at runtime. The `panic!` macro terminates the program with an error message — similar to throwing an uncaught exception. In Exercise 4, we'll learn about Rust's `Result` type for more graceful error handling.

## Your Task

👉 Open `exercises/src/exercise_2.rs` and implement the `transform` function using `match`.

Don't forget to run **`./serve.sh`** to open the frontend, start the backend and serve your exercises.

You'll want to support several filters from the [photon filters](https://docs.rs/photon-rs/latest/photon_rs/filters/index.html) module. Here are some to get you started:

- `"cali"` → `photon::filters::cali(img)`
- `"dramatic"` → `photon::filters::dramatic(img)`
- `"duotone_horizon"` → `photon::filters::duotone_horizon(img)`
- `"duotone_lilac"` → `photon::filters::duotone_lilac(img)`
- `"lofi"` → `photon::filters::lofi(img)`
- `"golden"` → `photon::filters::golden(img)`
- `"pastel_pink"` → `photon::filters::pastel_pink(img)`

👉 Test your implementation using the buttons below. Try different filter names!

Each line inside the `match` is called an **arm**: a pattern on the left, and code to run on the right, separated by `=>`. The `_` is a **wildcard** that matches anything (like `default:` in JavaScript).

Unlike JavaScript's `switch`, there's no fall-through and no `break` needed — each arm is self-contained. More importantly, Rust **requires** you to handle every possible case. Try removing the `_ => panic!(...)` line and the compiler will reject your code:

```text
error[E0004]: non-exhaustive patterns: `&_` not covered
```

This might feel strict, but it catches bugs at compile time that JavaScript would only reveal at runtime. The `panic!` macro crashes the program with an error message—similar to throwing an uncaught exception. In Exercise 4, we'll learn about Rust's `Result` type for more graceful error handling.

## Your Task

👉 Open `exercises/src/exercise_2.rs` and implement the `transform` function using `match`.

Don't forget to run **`./serve.sh`** to open the frontend, start the backend and serve your exercises.

You'll want to support several filters from the [photon filters module](https://docs.rs/photon-rs/latest/photon_rs/filters/index.html). Here are some to get you started:

- `"cali"` → `photon::filters::cali(&mut img)`
- `"dramatic"` → `photon::filters::dramatic(&mut img)`
- `"duotone_horizon"` → `photon::filters::duotone_horizon(&mut img)`
- `"lofi"` → `photon::filters::lofi(&mut img)`
- `"golden"` → `photon::filters::golden(&mut img)`
- `"pastel_pink"` → `photon::filters::pastel_pink(&mut img)`

👉 Test your implementation using the buttons below. Try different filter names!

<div class="workshop-exercise">
    <label for="imageUrl">Input image (URL)</label>
    <input type="text" class="workshop-image-url" id="imageUrl" placeholder="URL of source image" />
    <label for="filterSelect">Filter</label>
    <select id="filterSelect" class="workshop-select">
    <option value="cali">cali</option>
    <option value="dramatic">dramatic</option>
    <option value="duotone_horizon" selected>duotone_horizon</option>
    <option value="duotone_lilac">duotone_lilac</option>
    <option value="golden">golden</option>
    <option value="lofi">lofi</option>
    <option value="pastel_pink">pastel_pink</option>
    </select>
    <div class="workshop-buttons">
        <button class="workshop-btn workshop-btn-backend" onclick="triggerBackend('exercise_2', new URLSearchParams({ filter: document.getElementById('filterSelect').value }))">
            Backend
        </button>
        <button class="workshop-btn workshop-btn-wasm" onclick="triggerWasm('exercise_2', document.getElementById('filterSelect').value)">
            Wasm
        </button>
        {{#include includes/spinner.svg}}
    </div>
    <div class="workshop-output">
        <h4>Output <span id="timing-info"></span></h4>
        <div class="workshop-output--compare" style="overflow: visible;">
            <img id="imageOutput" class="workshop-output--compare__image-one">
            <div class="workshop-output--compare__mask">
                <img id="imageInput" class="workshop-output--compare__image-two" />
            </div>
            <div class="workshop-output--compare__separator">
                {{#include includes/slider-handle.svg}}
            </div>
            <input class="workshop-output--compare__input" type="range" min="0" step="0.5" max="100" value="50">
        </div>
        <div class="workshop-output--compare__labels"><p>Input image</p><p>Output image</p></div>
    </div>
</div>

## What's Next?

You've learned how `match` gives you type-safe branching with exhaustiveness checking—the compiler ensures you never forget a case. In the next exercise, we'll explore some other powerful Rust features: _arrays_, _loops_, and _iterators_.

{{#include includes/error-flash.html}}
