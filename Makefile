build:
	bootimage build --target x86_64-target.json
build-release:
	bootimage build --release --target x86_64-target.json
run:
	bootimage run
run-realease:
	bootimage run --release
test:
	cargo test
