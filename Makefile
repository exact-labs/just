clean:
	rm core_js

build:
	cargo build --release
	mv target/release/core_js .
   
test:
	cd tests && cargo run $(run)
