# Exercise 4: Parallel Iteration with Rayon

In the previous exercise, we stitched two images together. Now we'll take it further: create multiple thumbnails from a single image and stitch them all together **in parallel**. After all, your computer has multiple CPU cores, let's put them to work!

<div class="workshop-objectives">

### What you'll learn

- **Iterators**: Functional-style transformations with `.map()` and friends
- **Closures**: Rust's anonymous functions (similar to arrow functions)
- **Rayon**: A data-parallelism library that makes parallel iteration trivial
- **`reduce_with`**: Combining results from parallel computations

</div>

## The Goal

We'll build a transform function that takes in a single source image and produces multiple "thumbnails" at different resolutions. The resolutions we need to scale to are determined by a user-provided list of widths.

## Iterators: From `loop`s to `Iterator`s

In the previous chapter we've used `for` loops to copy pixels across and we could certainly use these again here to
produce the different thumbnails:

```rust
let mut thumbnails = Vec::new();
for width in widths {
    // push each resized image into the vec
    thumbnails.push(create_thumbnail(&img, *width));
}

fn create_thumbnail(src: &PhotonImage, width: u32) -> PhotonImage {
    // produce the resized image here
    todo!()
}
```

But Rust supports a more _elegant_ style of iteration called **Iterators**, a pattern you're likely already familiar with! Many languages has similar looking primitives. In JavaScript we might write this for example

```javascript
// iterate over the widths, create the resized thumbnail and collect everything into an array
const thumbnails = widths.map(width => createThumbnail(width));
```

The Rust equivalent looks remarkably similar:

```rust
// Rust
let thumbnails: Vec<_> = widths
    .iter()
    .map(|width| create_thumbnail(img, *width))
    .collect();
```

Notice the `iter()` and `collect()`. In Rust iterators are more flexible and not limited to Vecs/slices (in fact most collection types in Rust provide iterators!) so we'll need to _explicitly_ convert our slice into an iterator using `.iter()`.

We can then use any number of iterator functions (we call these _iterator combinators_). For a full list of functions check out the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#provided-methods) documentation!

Lastly, just like we had to turn our slice _into_ an iterator, we now need to turn our iterator _into_ a collection again. This is done by the
[`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) method. It will _collect_ all them items produced by the iterator into a collection type. In our case a regular `Vec`.

Now, there's one weird thing still: `|width| create_thumbnail(img, *width)` whats that??? what does `|width|` mean? Well, read right on!

## Closures

The `|width| ...` syntax defines a **closure**, an anonymous function that captures its surrounding scope.
Sounds scary but you're likely already familiar with this too, it's just like JavaScript's arrow functions, Ruby Closures or well, closures in any language really.

Closures must be assigned to a name and can access variables from their surrounding scope:

```rust
let multiplier = 3;
let multiply = |x| x * multiplier;  // Captures `multiplier`

println!("{}", multiply(5));  // Prints 15
```

## Combining with `.reduce()`

After mapping, we have multiple processed images. We need to combine them into one. That's what `.reduce()` does—it repeatedly combines elements from left to right until only one remains:

```
[A, B, C, D]
    ↓
[AB, C, D]
    ↓
[ABC, D]
    ↓
  [ABCD]
```

The closure receives two images and returns one combined image. Sound familiar? **This is exactly what your Exercise 3 code does!**

```rust
.reduce(|left, right| {
    // This is your transform() from Exercise 3
})
```

👉 Copy your image-stitching logic from Exercise 3 into the `reduce` closure.

Don't forget to run **`./serve.sh`** to open the frontend, start the backend and serve your exercises.

## Why `unwrap()`?

The `.reduce()` returns an `Option` because the input might be empty (nothing to reduce). The `.unwrap()` says "I know the input won't be empty."

| Input             | Result                 |
| ----------------- | ---------------------- |
| `[100, 200, 300]` | `Some(stitched_image)` |
| `[]` (empty)      | `None`                 |

## Your Task

👉 Open `exercises/src/exercise_4.rs` and implement the `transform` function using iterators and the iterator combinators provided by the standard library. Copy the code you wrote in Exercise 1 (for resizing images) and in Exercise 3 (for stitching two images together), you will need it!

Once you're done we'll look at parallelism and high performance code!

## Enter `rayon`

You might have wondered: `for` loops worked just fine, why do I need to bother with iterators?

[Rayon](https://docs.rs/rayon) is one of the most powerful Rust libraries. It allows you to **really easily**
add parallelism to your code. Just replace `.iter()` with `.par_iter()`, and Rayon handles the rest.

```rust
// Sequential: one CPU processes one element at a time
input.iter().map(|w| process(w)).collect()

// Parallel: all CPUs process elements in parallel
widths.par_iter().map(|w| process(w)).collect()
```

That's it. One method name changes, and your code runs across all available CPU cores. Rayon automatically creates a thread pool, distributes work, and joins results. This is incredibly powerful for modern CPUs where _the only way_ to get leading-edge performance is by going parallel. Rust makes this very easy AND safe!

### Why This Is Safe

Recall from chapter 2 that at any given time, you can have _either_ multiple immutable references _or_ one mutable reference, but **never both**. At the time you might have wondered why that restriction exists. Now is the time we'll
lift that secret.

`.par_iter()` will split up our input data into CPU-number of chunks and then process each chunk in parallel,
but you might worry: how is this safe? what if two threads modify the same data?

And this is exactly why Rust enforces this either-or relationship! It is very fine for multiple CPUs to be _reading_
the same data at the same time, but _writing_ the same data at the same time is not!

To illustrate this, consider a coffee machine: A 100 people can all _look_ at the coffee machine without issue, but if
a hundred people all try to _use_ the coffee machine at the same time we're going to have a problem (and a fight probably).

Rust's ownership system prevents this at compile time.

- Multiple threads can _read_ shared data (like our source image)
- No two threads can _write_ to the same data simultaneously

If you accidentally write code that would cause a data race, **the compiler rejects it**. You literally cannot compile a program with this class of bugs.

> Aside: Parallel Reduce
>
> Rayon also speeds up `.reduce()`! Instead of folding items one-by-one they will be reduced in parallel:
>
> ```
> [A, B, C, D]
>     ↓
> [AB,   CD]      (happens in parallel!)
>     ↓
>   [ABCD]
> ```

## Your Task (BONUS)

👉 Change your sequential-iterator solution in `exercises/src/exercise_4.rs` to use `.par_iter()`.
Consult the documentation for [rayons methods](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html) for help.

👉 **Challenge**: Add another `.map()` step to flip each thumbnail before stitching.

<div class="workshop-exercise">
    <label for="imageUrl">Input image (URL)</label>
    <input type="text" class="workshop-image-url" id="imageUrl" placeholder="URL of source image" value="https://c02.purpledshub.com/uploads/sites/41/2025/01/Naked-Mole-Rat.jpg?webp=1&w=1200" />
    <label for="imageWidths">Image widths (comma-separated)</label>
    <input type="text" class="workshop-image-param" id="imageWidths" placeholder="widths of thumbnail images" value="50, 100, 200, 400, 800, 1600" />
    <div class="workshop-buttons">
        <button class="workshop-btn workshop-btn-backend" onclick="{ triggerBackend('exercise_4', new URLSearchParams(document.getElementById('imageWidths').value.split(',').map(width => ['width', width.trim()]))); }">
            Run on Backend
        </button>
        <button class="workshop-btn workshop-btn-wasm" onclick="triggerWasm('exercise_4', document.getElementById('imageWidths').value.split(','))">
            Run in Wasm
        </button>
        {{#include includes/spinner.svg}}
    </div>
    <div class="workshop-output">
        <h4>Output <span id="timing-info"></span></h4>
        <div class="workshop-output--noncompare">
            <img id="imageOutput" />
        </div>
    </div>
</div>

{{#include includes/error-flash.html}}
