RUSTFLAGS="-Z sanitizer=address" cargo test -p rubtle-lib --lib -- --test-threads 1 --nocapture
