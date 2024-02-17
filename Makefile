.PHONY: clean fmt test help

clean:
	cargo clean

build:
	cargo build --release

fmt:
	cargo fmt --all -- --check

test:
	cargo test

help:
	@echo "build - compile the project"
	@echo "clean - remove the target directory"
	@echo "fmt - format the code"
	@echo "test - excute all unit tests"
	@echo "help - print this help message"
