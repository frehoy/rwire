dev : 
	cargo fmt
	cargo clippy
	cargo test

run : 
	cargo run

full : dev
	cargo audit
