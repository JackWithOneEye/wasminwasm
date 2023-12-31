#!/bin/sh

mkdir -p ./wasm
# cp $(tinygo env TINYGOROOT)/targets/wasm_exec.js ./wasm/
# tinygo build -o ./wasm/go.wasm -target=wasm ./go/cmd/main.go
cp $(go env GOROOT)/misc/wasm/wasm_exec.js ./wasm/
GOOS=js GOARCH=wasm go build -o ./wasm/go.wasm ./go/cmd/main.go

cd zig
zig build-lib src/export.zig -target wasm32-freestanding -dynamic -rdynamic -O ReleaseSmall --name zig
mv zig.wasm ../wasm/zig.wasm