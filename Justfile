set positional-arguments

wasm:
  wasm-pack build --target web --release -- --features wasm

run *args='':
  cargo run --release --example today-is -- $@
