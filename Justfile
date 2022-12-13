set positional-arguments

wasm:
  wasm-pack build --target web --release --out-dir public/pkg -- --features wasm

run *args='':
  cargo run --release --example today-is -- $@

serve:
  miniserve --index index.html -- public

lint:
  cargo clippy --all-features --all-targets -- -D warnings
