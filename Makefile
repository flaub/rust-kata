.PHONY: all test watch

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
