# Exercise 3

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

<div class="workshop-exercise">

<input type="text" class="workshop-image-url" id="imageUrlLeft" placeholder="URL of source image" value="https://picsum.photos/1800/1600" />
<input type="text" class="workshop-image-url" id="imageUrlRight" placeholder="URL of source image" value="https://picsum.photos/1800/1600" />

<div class="workshop-buttons">
<button class="workshop-btn workshop-btn-backend" onclick="triggerBackendExercise3()">
Backend
</button>
<button class="workshop-btn workshop-btn-wasm" onclick="triggerWasmExercise3()">
Wasm
</button>
</div>

<div class="workshop-output">
<h4>Output</h4>
<img id="imageOutput" src="" alt="Transformed image will appear here" />
</div>

</div>
