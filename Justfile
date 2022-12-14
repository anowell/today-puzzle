set positional-arguments

wasm:
  wasm-pack build --target web --release --out-dir public/pkg -- --features wasm

run *args='':
  cargo run --release --example today-is -- $@

serve:
  miniserve --index index.html -- public

lint *args='':
  cargo clippy --fix --all-features --all-targets $@ -- -D warnings

flamegraph:
  #!/usr/bin/env sh
  export CARGO_PROFILE_RELEASE_DEBUG=true
  echo $CARGO_PROFILE_RELEASE_DEBUG
  cargo build --release
  cargo flamegraph --example today-is -- -v crea-makerspace -p count
