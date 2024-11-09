run:
        sudo cargo run --release --config "target.'cfg(target_arch = \"avr\")'.runner = 'ravedude -cb 57600 mega2560'"  --target="./avr-atmega2560.json" -Zbuild-std="core"

test:
        cargo test 