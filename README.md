# Request the lib to be unloaded to free the dll file lock
cargo watch -w rustpy-dylib -s 'curl http://localhost:8000/reload?lib=rustpy-dylib'
# Compile the lib and replace the dll when confirmation received
**WIP** cargo watch -w rustpy-dylib -s 'touch target/debug/rustpy_dylib.dll' -x 'build'
