# Exercise 2: Pattern Matching with `match`

This exercise introduces one of Rust's most powerful features: pattern matching with `match`.

<div class="workshop-objectives">

### What you'll learn

- **Pattern matching**: Rust's `match` expression
- **Exhaustiveness**: Why Rust forces you to handle all cases
- **The `_` wildcard**: A catch-all pattern

</div>

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

## `match` vs `switch`

If you're familiar with JavaScript's `switch`, `match` will feel similar—but with important differences:

<table class="workshop-comparison">
<thead>
<tr>
<th>JavaScript <code>switch</code></th>
<th>Rust <code>match</code></th>
</tr>
</thead>
<tbody>
<tr>
<td>Falls through by default</td>
<td>No fall-through</td>
</tr>
<tr>
<td><code>default:</code> is optional</td>
<td>Must handle all cases</td>
</tr>
<tr>
<td>Can forget <code>break</code></td>
<td>Each arm is self-contained</td>
</tr>
</tbody>
</table>

The `_` pattern is Rust's catch-all, similar to `default:` in JavaScript. Here we use `panic!` to crash if an unknown filter is requested—we'll see better error handling patterns later.

## String slices: `&str`

Notice the parameter type is `&str`, not `String`. In Rust:

- `String` is an owned, heap-allocated string (like a JavaScript string)
- `&str` is a _borrowed_ reference to string data

For now, think of `&str` as "a string I can read but don't own."

<div class="workshop-exercise">

<input type="text" class="workshop-image-url" id="imageUrl" placeholder="URL of source image" value="https://picsum.photos/1800/1600" />

<div class="workshop-buttons">
<button class="workshop-btn workshop-btn-backend" onclick="triggerBackend('exercise_2', { filter: 'duotone_horizon' })">
Backend
</button>
<button class="workshop-btn workshop-btn-wasm" onclick="triggerWasm('exercise_2', 'duotone_horizon')">
Wasm
</button>
</div>

<div class="workshop-output">
<h4>Output</h4>
<img id="imageOutput" src="" alt="Transformed image will appear here" />
</div>

</div>
