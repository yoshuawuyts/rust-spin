all:
	rustc index.rs
	./index

test:
	rustc --test test.rs
	./test

clean:
	rm index
	rm test

.PHONY: test
