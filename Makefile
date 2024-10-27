release:
	cargo build --release

image:
	docker build .
