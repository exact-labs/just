clean:
	cd src/embed && rm external
	rm core_js

build:
	cd src/embed && go mod tidy && go build .
	cargo build --release
	mv target/release/core_js .
   
test:
	cd src/embed && go mod tidy && go build .
	cd tests && cargo run run $(run).js

test_all_run:
	cd src/embed && go mod tidy && go build .
	d=$$(date +%s)\
	; for file in tests/*.js; do \
		cd tests; \
		cargo run run $${file#*/}; \
		cd ../; \
	done \
	&& echo "\n\033[4;36m\033[1;36mtests took $$(($$(date +%s)-d)) seconds\033[0m"

test_all_build:
	cd src/embed && go mod tidy && go build .
	cargo build --release
	mv target/release/core_js .
	d=$$(date +%s)\
	; for file in tests/*.js; do \
		cd tests; \
		../core_js run $${file#*/}; \
		cd ../; \
	done \
	&& echo "\n\033[4;36m\033[1;36mtests took $$(($$(date +%s)-d)) seconds\033[0m"