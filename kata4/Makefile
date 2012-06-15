all: bin/data

bin/data: bin data.rs
	rustc data.rs -o $@

bin/test: bin data.rs
	rustc data.rs -o $@ --test
	
bin:
	mkdir bin
	
test: bin/test
	bin/test
	
clean:
	rm -rf bin
