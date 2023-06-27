mkdir ./wasm
cp $(tinygo env TINYGOROOT)/targets/wasm_exec.js ./wasm/
tinygo build -o ./wasm/go.wasm -target=wasm ./go/cmd/main.go

cd zig
zig build-lib -O ReleaseSafe -target wasm32-freestanding --name zig -dynamic src/export.zig
mv zig.wasm ../wasm/zig.wasm