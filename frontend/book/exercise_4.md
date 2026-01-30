# Exercise 4

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

| JavaScript                   | Rust                 |
| ---------------------------- | -------------------- |
| `(x) => x * 2`               | `\|x\| x * 2`        |
| `(x, y) => { return x + y }` | `\|x, y\| { x + y }` |

### Why parallelism matters

WebAssembly can now use multiple threads. This means CPU-intensive tasks like image processing can run significantly faster. Rayon makes this almost trivial—just replace `.iter()` with `.par_iter()` and Rayon handles the rest.

### The `reduce_with` pattern

```rust
.reduce_with(|left, right| {
    // Combine two images into one
})
```

This is similar to JavaScript's `.reduce()`, but designed for parallel execution. It takes pairs of results and combines them until only one remains—perfect for stitching images together.

<div class="workshop-exercise">
    <input type="text" class="workshop-image-url" id="imageUrl" placeholder="URL of source image" value="https://picsum.photos/1800/1600" />
    <div class="workshop-buttons">
        <button class="workshop-btn workshop-btn-backend" onclick="triggerBackend('exercise_4', {})">
            Backend
        </button>
        <button class="workshop-btn workshop-btn-wasm" onclick="triggerWasm('exercise_4')">
            Wasm
        </button>
    </div>
    <div class="workshop-output">
        <h4>Output</h4>
        <img id="imageOutput" src="" alt="Transformed image will appear here" />
    </div>
</div>
