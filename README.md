# Request the lib to be unloaded to free the dll file lock
cargo watch -w rustpy-dylib -s 'curl http://localhost:8000/reload?lib=rustpy-dylib'
