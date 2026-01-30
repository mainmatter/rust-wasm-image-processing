# Exercise 1: Your First Transformation

In this exercise, you'll write your first Rust function—a simple image transformation that flips an image vertically.

<div class="workshop-objectives">

### What you'll learn

- **Function signatures**: How Rust functions declare their inputs and outputs.
- **Mutability**: The difference between `mut` and immutable values.
- **Rust Crates**: How to install and use 3rd party packages.

</div>

We want to apply a vertical flip on the input image.
To do so we are going to use the [`flipv`](https://docs.rs/photon-rs/latest/photon_rs/transform/fn.flipv.html)
function located in the [`transform module`](https://docs.rs/photon-rs/latest/photon_rs/transform/index.html)
of the [`photon crate`](https://docs.rs/photon-rs/latest/photon_rs/index.html).

A Crate is a Rust external library. They are usually published on [`crates.io`](https://crates.io/) - a package registry like npm, PyPI - which
currently hosts more than 200,000 different crates!
The documentation of crates can usually be found on [docs.rs](https://docs.rs/),
it's very useful to check the API of a crate you are considering to use in your project.
See for example [the photon page on crates.io](https://crates.io/crates/photon-rs)
and [its documentation on docs.rs](https://docs.rs/photon-rs/latest/photon_rs/).

Let's implement our first filter by editing `transformers/src/exercise_1.rs`:

```rust
pub fn transform(mut img: PhotonImage) -> PhotonImage {
    todo!()
}
```

Let's break it down together:

- `pub fn` are two Rust keywords used to define a public function. We need to make it public as this function will be called from other another file.
- This function takes a single `PhotonImage` argument and returns the modified image using the same type.
- Coming from JavaScript, you might be used to modifying objects freely. Rust is stricter: values are **immutable by default**. You must explicitly mark something as `mut` if you intend to change it.
- This function is not implemented yet so we use the `todo!()` macro as a placeholder. Trying to execute this code will result in a panic at runtime.

👉 Let's try to execute it by clicking on the `Backend` button below. You should see something like this in the terminal running the server:

```
thread 'tokio-runtime-worker' (154320) panicked at transformers/src/exercise_1.rs:11:5:
not yet implemented
```

We can now implement the image flip using [`flipv`](https://docs.rs/photon-rs/latest/photon_rs/transform/fn.flipv.html)
by calling `photon::transform::flipv(&mut img);` and then returning the modified image.

👉 Modify the function core in `transformers/src/exercise_1.rs` and test it by clicking on the `Backend` and `Frontend` buttons.

You should see the image being flipped vertically.

👉 You can try other transformations from the [transform](https://docs.rs/photon-rs/latest/photon_rs/transform/index.html) module and
check how they affect the image.

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
