# Android fileopen dialog

![fileopen](https://user-images.githubusercontent.com/910977/177209193-e9f84605-c210-473a-9017-76bf9884452.gif)

Code from "How to write Java with miniquad" tutorial.
Showcases miniquad<->java interop.
More of a Java native plugin example than a real usable crate!

## How to use

- Add a "quad-fileopen" crate as a dependency to Cargo.toml:

```toml
[dependencies]
quad-bt = { path = "../quad-fileopen" } # functionality of example-android-fileopen would probably be not sufficient, so local copy for local hacks is preferable!
```

- call `fileopen::find_file` from the main loop: 

```rust
let file_data = Arc::new(Mutex::new(None));

loop {
    if root_ui().button(None, "Open file"){
        fileopen::find_file(file_data.clone());
    }
}
```

for the full example check "examples/fileopen.rs"

