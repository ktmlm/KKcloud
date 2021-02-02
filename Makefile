
TARGET = $(shell rustup toolchain list | grep default | sed -r 's/^ *[^-]+-//' | sed 's/ *(default).*//')

all: build

build:
	cargo build --target=$(TARGET)

release:
	cargo build --release --target=$(TARGET)

lint:
	@ cargo clippy

test:
	cargo test --workspace -- --test-threads=1
	cargo test --lib --workspace -- --test-threads=1

fmt:
	@ ./tools/fmt.sh

clean:
	@ find . -type f -name "Cargo.lock" | xargs rm -f

cleanall: clean
	@ git clean -fdx
	@ cargo clean

doc:
	@ cargo doc --workspace
	@ cd src/engine && cargo doc --open
	@ cd src/framework && cargo doc --open
	@ cd src/server && cargo doc --open
	@ cd src/client && cargo doc --open
	@ cd src/kkcloud && cargo doc --open
