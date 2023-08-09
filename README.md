# Cancellable JS promise in Rust

## Usage

```rust
let promise = wasm_bindgen_futures::future_to_promise(async move {
    ...
});

let abort_controller = Box::new(worker::AbortController::default());
let output = cancellable_promise::make(abort_controller.signal(), promise);

// eventually ...

abort_controller.abort();
```
