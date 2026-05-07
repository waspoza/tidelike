RUSTFLAGS='-C target-cpu=native -Zlocation-detail=none -Zfmt-debug=none -Zunstable-options -Cpanic=immediate-abort' \
cargo +nightly bloat \
-Z build-std=std,panic_abort \
-Z build-std-features="optimize_for_size" \
--target x86_64-unknown-linux-gnu --release   
