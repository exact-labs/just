clean:
	rm core_js

build:
	cargo build --release
	mv target/release/core_js .
   
test:
	cd tests && cargo run $(run)

test_all_run:
	d=$$(date +%s)\
	; for file in tests/*.js; do \
		cd tests; \
		a=$${file#*/}; \
		cargo run $${a%.js*}; \
		cd ../; \
	done \
	&& echo "\n\033[4;36m\033[1;36mtests took $$(($$(date +%s)-d)) seconds\033[0m"

test_all_build:
	cargo build --release
	mv target/release/core_js .
	d=$$(date +%s)\
	; for file in tests/*.js; do \
		cd tests; \
		a=$${file#*/}; \
		../core_js $${a%.js*}; \
		cd ../; \
	done \
	&& echo "\n\033[4;36m\033[1;36mtests took $$(($$(date +%s)-d)) seconds\033[0m"