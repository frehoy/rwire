dev : 
	cargo fmt
	cargo clippy -- -D warnings
	cargo test

run : 
	cargo run

full : dev
	cargo audit
