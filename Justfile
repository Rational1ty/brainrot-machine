default: build start

build:
	cargo build

release:
	cargo build --release

start:
	./target/debug/brainrot-machine.exe
