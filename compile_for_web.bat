cargo build --target wasm32-unknown-unknown -r

ROBOCOPY target\wasm32-unknown-unknown\release web\ *.wasm

rem cargo build --target wasm32-unknown-unknown

rem ROBOCOPY target\wasm32-unknown-unknown\debug web\ *.wasm