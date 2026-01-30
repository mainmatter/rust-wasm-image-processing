# Exercise Descriptions

## Exercise 1: Your First Transformation

In this exercise, you'll write your first Rust function—a simple image transformation that flips an image vertically.

### What you'll learn

- **Function signatures**: How Rust functions declare their inputs and outputs
- **Mutability**: The difference between `mut` and immutable values

### The code

```rust
pub fn transform(mut img: PhotonImage) -> PhotonImage {
    photon::transform::flipv(&mut img);
    img
}
```

Let's break this down:

- `pub fn transform(...)` declares a public function named `transform`
- `mut img: PhotonImage` means "this function takes a mutable image as input"
- `-> PhotonImage` means "this function returns an image"
- `&mut img` passes a *mutable reference* to the `flipv` function

### Mutability in Rust

Coming from JavaScript, you might be used to modifying objects freely. Rust is stricter: values are **immutable by default**. You must explicitly mark something as `mut` if you intend to change it.

Think of it like `const` being the default in JavaScript, except Rust enforces it at compile time.

---

## Exercise 2: Pattern Matching with `match`

This exercise introduces one of Rust's most powerful features: pattern matching with `match`.

### What you'll learn

- **Pattern matching**: Rust's `match` expression
- **Exhaustiveness**: Why Rust forces you to handle all cases
- **The `_` wildcard**: A catch-all pattern

### The code

```rust
pub fn transform(mut img: PhotonImage, filter: &str) -> PhotonImage {
    match filter {
        "cali" => photon::filters::cali(&mut img),
        "dramatic" => photon::filters::dramatic(&mut img),
        // ... more filters ...
        _ => panic!("no such filter"),
    }
    img
}
```

### `match` vs `switch`

If you're familiar with JavaScript's `switch`, `match` will feel similar—but with important differences:

| JavaScript `switch`          | Rust `match`                     |
|------------------------------|----------------------------------|
| Falls through by default     | No fall-through                  |
| `default:` is optional       | Must handle all cases            |
| Can forget `break`           | Each arm is self-contained       |

The `_` pattern is Rust's catch-all, similar to `default:` in JavaScript. Here we use `panic!` to crash if an unknown filter is requested—we'll see better error handling patterns later.

### String slices: `&str`

Notice the parameter type is `&str`, not `String`. In Rust:
- `String` is an owned, heap-allocated string (like a JavaScript string)
- `&str` is a *borrowed* reference to string data

For now, think of `&str` as "a string I can read but don't own."

---

## Exercise 3: Collections and Iteration

Now things get more interesting. This exercise creates a thumbnail strip by resizing an image to multiple widths and stitching the results together.

### What you'll learn

- **Vectors**: Rust's growable array type, `Vec<T>`
- **Slices**: Borrowed views into contiguous data, `&[T]`
- **`for` loops**: Iterating over collections

### Vectors and slices

```rust
pub fn transform(img: PhotonImage, widths: &[u32]) -> PhotonImage {
    let mut thumbnails = Vec::new();

    for width in widths {
        // ... create thumbnail ...
        thumbnails.push(thumbnail);
    }
    // ...
}
```

The parameter `widths: &[u32]` is a **slice**—a borrowed view into a sequence of `u32` values. This is similar to how JavaScript functions often accept arrays without caring whether they're the original or a copy.

`Vec::new()` creates an empty vector. Unlike JavaScript arrays, Rust vectors are:
- **Typed**: A `Vec<PhotonImage>` can only hold `PhotonImage` values
- **Growable**: Use `.push()` to add elements

### The `for` loop

```rust
for width in widths {
    // width is borrowed from the slice
}
```

This works like JavaScript's `for...of` loop. Rust automatically handles the iteration for you.

### Working with pixels

The exercise manually copies pixels between images. Don't worry about memorizing this—it's showing you that Rust gives you low-level control when you need it, while still keeping you safe from memory bugs.

---

## Exercise 4: Parallel Iteration with Rayon

The final exercise takes everything you've learned and adds parallelism. You'll process multiple thumbnails simultaneously using Rayon, a data-parallelism library.

### What you'll learn

- **Iterator combinators**: Chaining `.map()`, `.filter()`, and friends
- **Parallel iteration**: Using Rayon's `.par_iter()`
- **`reduce`**: Combining results from parallel computations

### From `for` loops to iterators

In Exercise 3, we used a `for` loop. Rust also supports a functional style with iterator combinators:

```rust
widths
    .par_iter()                    // Iterate in parallel
    .map(|width| { /* resize */ }) // Transform each element
    .map(|mut image| { /* filter */ })
    .reduce_with(|left, right| { /* stitch */ })
    .unwrap()
```

If you've used JavaScript's `.map()` and `.reduce()`, this will feel familiar. The key difference: `.par_iter()` processes elements **across multiple CPU cores**.

### Closures

The `|width| { ... }` syntax defines a **closure**—an anonymous function. It's equivalent to JavaScript's arrow functions:

| JavaScript                | Rust                          |
|---------------------------|-------------------------------|
| `(x) => x * 2`            | `\|x\| x * 2`                 |
| `(x, y) => { return x + y }` | `\|x, y\| { x + y }`       |

### Why parallelism matters

WebAssembly can now use multiple threads. This means CPU-intensive tasks like image processing can run significantly faster. Rayon makes this almost trivial—just replace `.iter()` with `.par_iter()` and Rayon handles the rest.

### The `reduce_with` pattern

```rust
.reduce_with(|left, right| {
    // Combine two images into one
})
```

This is similar to JavaScript's `.reduce()`, but designed for parallel execution. It takes pairs of results and combines them until only one remains—perfect for stitching images together.
