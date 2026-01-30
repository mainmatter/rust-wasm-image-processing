# Exercise 1: Your First Transformation

In this exercise, you'll write your first Rust function—a simple image transformation that flips an image vertically.

<div class="workshop-objectives">

### What you'll learn

- **Function signatures**: How Rust functions declare their inputs and outputs
- **Mutability**: The difference between `mut` and immutable values

</div>

```rust
pub fn transform(mut img: PhotonImage) -> PhotonImage {
    photon::transform::flipv(&mut img);
    img
}
```

Coming from JavaScript, you might be used to modifying objects freely. Rust is stricter: values are **immutable by default**. You must explicitly mark something as `mut` if you intend to change it.

<div class="workshop-exercise">

<input type="text" class="workshop-image-url" id="imageUrl" placeholder="URL of source image" value="https://picsum.photos/1800/1600" />

<div class="workshop-buttons">
<button class="workshop-btn workshop-btn-backend" onclick="triggerBackend('exercise_1', {}, 'output-ex1')">
Backend
</button>
<button class="workshop-btn workshop-btn-wasm" onclick="triggerWasm('exercise_1', 'output-ex1')">
Wasm
</button>
</div>

<div class="workshop-output">
<h4>Output</h4>
<img id="imageOutput" src="" alt="Transformed image will appear here" />
</div>

</div>
