
TARGET = $(shell rustup toolchain list | grep default | sed -r 's/^ *[^-]+-//' | sed 's/ *(default).*//')

all: build

build:
	cargo build --target=$(TARGET)

release:
	cargo build --release --target=$(TARGET)

lint:
	@ cargo clippy

fmt:
	@ ./tools/fmt.sh

clean:
	@ find . -type f -name "Cargo.lock" | xargs rm -f

cleanall: clean
	@ git clean -fdx
	@ cargo clean

doc:
	cargo doc --open
