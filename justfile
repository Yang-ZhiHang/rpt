set shell := ["cmd", "/c"]

run:
    cargo run

generate:
    cargo run > output/image.ppm