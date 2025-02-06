run:
        RUSTFLAGS="-Zlocation-detail=none" cargo run --release --config "target.'cfg(target_arch = \"avr\")'.runner = 'ravedude -cb 57600 uno'"  --target="./board_specs/avr-atmega328p.json" -Zbuild-std="core,panic_abort"  -Z build-std-features="optimize_for_size,panic_immediate_abort"
build:
        cargo build --release --config "target.'cfg(target_arch = \"avr\")'.runner = 'ravedude -cb 57600 uno'"  --target="./board_specs/avr-atmega328p.json" -Zbuild-std="core" 

builddbg:
        cargo build --config "target.'cfg(target_arch = \"avr\")'.runner = 'ravedude -cb 57600 uno'"  --target="./build_specs/avr-atmega328p.json" -Zbuild-std="core"

test:
        cargo test 
