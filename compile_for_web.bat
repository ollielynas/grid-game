rem cargo build --target wasm32-unknown-unknown -r

rem ROBOCOPY target\wasm32-unknown-unknown\release web\ *.wasm

cargo build --target wasm32-unknown-unknown

ROBOCOPY target\wasm32-unknown-unknown\debug web\ *.wasm