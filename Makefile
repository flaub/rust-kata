.PHONY: all test watch clean

all:
	cargo build

test:
	@echo
	@echo
	@echo -------------------- 
	@echo 
	@echo 
	cargo test -- --nocapture

watch:
	ls src/*.rs | entr make test

clean:
	cargo clean
