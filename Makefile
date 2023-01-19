clean:
	cd crates/go/src/embed && rm external
	rm just

build:
	cd crates/go/src/embed && go mod tidy && go build .
	cargo build --release
	mv target/release/just .

publish:
	cd crates/go && cargo publish --no-verify
	cd crates/macros && cargo publish --no-verify
	cd crates/state && cargo publish --no-verify

test:
	cd crates/go/src/embed && go mod tidy && go build .
	cd tests/javascript && cargo run run $(run).js

test_all_run:
	cd crates/go/src/embed && go mod tidy && go build .
	d=$$(date +%s)\
	; for file in tests/javascript/*.js; do \
		cd tests; \
		cargo run run $${file#*/}; \
		cd ../; \
	done \
	&& echo "\n\033[4;36m\033[1;36mtests took $$(($$(date +%s)-d)) seconds\033[0m"

test_all_build:
	cd crates/go/src/embed && go mod tidy && go build .
	cargo build --release
	mv target/release/just .
	d=$$(date +%s)\
	; for file in tests/javascript/*.js; do \
		cd tests; \
		../just run $${file#*/}; \
		cd ../; \
	done \
	&& echo "\n\033[4;36m\033[1;36mtests took $$(($$(date +%s)-d)) seconds\033[0m"