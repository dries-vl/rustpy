@echo off
:: set python executable to a full distribution of python for building
set "PYTHON_SYS_EXECUTABLE=%~dp0python-3.11.5-build/python.exe"

:: use sccache to use cached parts of builds to speed up total compilation time
:: set "RUSTC_WRAPPER=sccache"

:: add no location details to binary to reduce size (requires nightly)
set "RUSTFLAGS=%RUSTFLAGS% -Z location-detail=none"

:: build for release to enable release optimisations in cargo.toml file (slower compilation)
:: build the std library from scratch to reduce size (requires nightly and specifying target)
:: do not include panic strings and formatting code in final binary
cargo +nightly build --release --target x86_64-pc-windows-msvc -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
