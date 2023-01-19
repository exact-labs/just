clean:
	rm just

build:
	cargo build --release
	mv target/release/just .

publish:
	cd crates/macros && cargo publish --no-verify
	cd crates/state && cargo publish --no-verify

test:
	cd tests/javascript && cargo run run $(run).js

test_all_run:
	d=$$(date +%s)\
	; for file in tests/javascript/*.js; do \
		cd tests; \
		cargo run run $${file#*/}; \
		cd ../; \
	done \
	&& echo "\n\033[4;36m\033[1;36mtests took $$(($$(date +%s)-d)) seconds\033[0m"

test_all_build:
	cargo build --release
	mv target/release/just .
	d=$$(date +%s)\
	; for file in tests/javascript/*.js; do \
		cd tests; \
		../just run $${file#*/}; \
		cd ../; \
	done \
	&& echo "\n\033[4;36m\033[1;36mtests took $$(($$(date +%s)-d)) seconds\033[0m"