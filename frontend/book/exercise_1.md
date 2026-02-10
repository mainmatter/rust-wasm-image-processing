# Exercise 1: Your First Transformation

In this exercise, you'll write your first Rust function — a transformation that resizes an image to a given width while maintaining its aspect ratio.

<div class="workshop-objectives">

### What you'll learn

- **Function signatures**: How Rust functions declare their inputs and outputs.
- **Mutability**: The difference between `mut` and immutable values.
- **Number types**: Rust's numeric type system and explicit conversions
- **Rust Crates**: How to install and use 3rd party packages.

</div>

We want to resize an input image to a specified width, calculating the appropriate height to maintain the original aspect ratio. To do this, we'll use the [`resize`](https://docs.rs/photon-rs/latest/photon_rs/transform/fn.resize.html) function from the [`transform`](https://docs.rs/photon-rs/latest/photon_rs/transform/index.html) module of the [`photon`](https://docs.rs/photon-rs/latest/photon_rs/index.html) crate.

A **crate** is a Rust package, similar to npm packages or Python modules. Crates are published to [`crates.io`](https://crates.io/) — a package registry hosting over 200,000 packages. Rust automatically generates documentation for _all_ published crates at [docs.rs](https://docs.rs/), which is useful for exploring a crate's API before using it. See [the photon page on crates.io](https://crates.io/crates/photon-rs) and [its documentation on docs.rs](https://docs.rs/photon-rs/latest/photon_rs/).

Let's implement our first transformation by editing `exercises/src/exercise_1.rs`:

```rust
pub fn transform(img: &PhotonImage, target_width: u32) -> PhotonImage {
    todo!()
}
```

Let's break this down together:

This function takes a single `PhotonImage` argument and returns the modified image using the same type.

`pub fn` are two Rust keywords used to define a public function. We need to make it public as this function will be called from another file.

Coming from JavaScript, you might be used to modifying objects freely. Rust is stricter: values are **immutable by default**. You must explicitly mark something as `mut` if you intend to change it.

The `todo!()` macro is a placeholder. Running this code will panic at runtime with "not yet implemented".

👉 Try it now by clicking the **Backend** button below. You should see something like this in the terminal:

```
thread 'tokio-runtime-worker' panicked at exercises/src/exercise_1.rs:5:5:
not yet implemented
```

## Number Types in Rust

Before implementing the resize, let's talk about numbers. The function takes a parameter `width: u32` the target width as an unsigned 32-bit integer.
Unlike JavaScript (which has a single `number` type), Rust has distinct numeric types:

| Type                      | Description       | Range           |
| ------------------------- | ----------------- | --------------- |
| `u8`, `u16`, `u32`, `u64` | Unsigned integers | 0 to 2ⁿ-1       |
| `i8`, `i16`, `i32`, `i64` | Signed integers   | -2ⁿ⁻¹ to 2ⁿ⁻¹-1 |
| `f32`, `f64`              | Floating-point    | IEEE 754        |

The `u` prefix means unsigned (no negative values), `i` means signed, and `f` means floating-point. The number indicates the bit width.

Rust **won't implicitly convert** between numeric types. If you have a `u32` and need an `f32`, you must explicitly cast it:

```rust
let width: u32 = 800;
let width_float: f32 = width as f32;  // Explicit cast required
```

This strictness prevents subtle bugs. In JavaScript, `0.1 + 0.2 !== 0.3` can surprise you. In Rust, you always know exactly what numeric operations you're performing.

## Implementing the Resize

To resize while maintaining aspect ratio, we need to:

1. Calculate the original aspect ratio (width ÷ height)
2. Divide the new width by this ratio to get the new height
3. Call the `resize` function

```rust
use photon::transform::{resize, SamplingFilter};

// Calculate aspect ratio using floating-point math
let aspect_ratio = img.get_width() as f32 / img.get_height() as f32;
let target_height = (target_width as f32 / aspect_ratio) as u32;

// Resize and return
resize(&img, target_width, target_height, SamplingFilter::Nearest)
```

Notice how we cast to `f32` for the division (to avoid integer truncation), then cast the result back to `u32`.

The `SamplingFilter::Nearest` argument specifies the resampling algorithm—`Nearest` is fast, while other options like `Lanczos3` produce smoother results. The `::` syntax accesses a variant of an **enum** (enumeration)—we'll explore enums in Exercise 2.

👉 Implement the `transform` function in `exercises/src/exercise_1.rs` and test it by clicking the **Backend** and **Wasm** buttons.

Run **`./serve.sh`** to open the frontend, start the backend and serve your exercises.

👉 Try different width values to see the resizing in action. Compare how small widths (like 100) versus large widths (like 1600) affect the result.

<div class="workshop-exercise">
    <label for="imageUrl">Input image (URL)</label>
    <input type="text" class="workshop-image-url" id="imageUrl" placeholder="URL of source image" />
    <label for="targetWidth">Target width</label>
    <input type="number" class="workshop-image-param" id="targetWidth" placeholder="Target width" value="400" min="1" max="4000" />
    <div class="workshop-buttons">
        <button class="workshop-btn workshop-btn-backend" onclick="triggerBackend('exercise_1', new URLSearchParams({ width: document.getElementById('targetWidth').value }))">
            Backend
        </button>
        <button class="workshop-btn workshop-btn-wasm" onclick="triggerWasm('exercise_1', parseInt(document.getElementById('targetWidth').value))">
            Wasm
        </button>
        {{#include includes/spinner.svg}}
    </div>
    <div class="workshop-output">
        <h4>Output <span id="timing-info"></span></h4>
        <div class="workshop-output--compare" style="overflow: visible;">
            <img id="imageOutput" class="workshop-output--compare__image-one original-size">
            <div class="workshop-output--compare__mask">
                <img id="imageInput" class="workshop-output--compare__image-two original-size" />
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

You've implemented your first Rust function and learned about Rust's strict numeric type system. Next, we'll explore pattern matching with `match`—one of Rust's most powerful features for handling different cases safely.

{{#include includes/error-flash.html}}
