run:
        RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug" sudo cargo run --release --config "target.'cfg(target_arch = \"avr\")'.runner = 'ravedude -cb 57600 mega2560'"  --target="./board_specs/avr-atmega2560.json" -Zbuild-std="core,panic_abort"  -Z build-std-features="optimize_for_size,panic_immediate_abort"
build:
        cargo build --release --config "target.'cfg(target_arch = \"avr\")'.runner = 'ravedude -cb 57600 mega2560'"  --target="./board_specs/avr-atmega2560.json" -Zbuild-std="core" 

builddbg:
        cargo build --config "target.'cfg(target_arch = \"avr\")'.runner = 'ravedude -cb 57600 mega2560'"  --target="./build_specs/avr-atmega2560.json" -Zbuild-std="core"

test:
        cargo test 
