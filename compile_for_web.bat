cargo build --target wasm32-unknown-unknown -r

ROBOCOPY target\wasm32-unknown-unknown\release web\ *.wasm