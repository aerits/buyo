cargo build --bin client --target wasm32-unknown-unknown
mkdir client_app_output/static
cp -r assets client_app_output/static/
cp target/debug/web client_app_output/web
wasm-bindgen target/wasm32-unknown-unknown/debug/client.wasm --out-dir client_app_output/static --target web
python generate_html.py