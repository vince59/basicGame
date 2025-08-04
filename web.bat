rem pour déploiement en localhost
cargo build --release --target wasm32-unknown-unknown
copy .\target\wasm32-unknown-unknown\release\basicGame.wasm .\www\
copy index.html .\www\
copy assets\*.* .\www\assets
copy quad-storage.js .\www\
copy sapp_jsutils.js .\www\
cd .\www
python -m http.server 8000
