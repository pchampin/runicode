runicode.h: target/debug/librunicode.a
	cbindgen --config cbindgen.toml --crate runicode --output "$@"
	touch "$@"

target/debug/librunicode.a: src/*.rs
	cargo build

.PHONY=test
test: test_c/test_c
	./test_c/test_c héllø world ⛄

test_c/test_c: test_c/main.c runicode.h
	cc test_c/main.c target/debug/librunicode.a -o "$@"
