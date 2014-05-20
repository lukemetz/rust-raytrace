all: lib test

lib: src/lib.rs Makefile src/*.rs
	rustc src/lib.rs

test: src/lib.rs Makefile src/*.rs
	rustc src/lib.rs --test -o test

vis_test: intersection bsdf

intersection: lib vis_tests/intersection.rs
	rustc -L . vis_tests/intersection.rs

bsdf: lib vis_tests/bsdf.rs
	rustc -L . vis_tests/bsdf.rs

.PHONY: clean

clean:
	rm -f intersection bsdf
	rm -f test
	rm -f libraytrace_core*.rlib
	rm -f intersection.ppm bsdf.ppm
