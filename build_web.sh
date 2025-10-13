cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "vamita" \
    ./target/wasm32-unknown-unknown/release/vamita.wasm

# cp -r assets out/
ln -s ../assets out/assets
cp index.html out/
