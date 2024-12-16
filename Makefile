rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

cleanup:
	@echo "Cleaning up target"
	rm -rf target	
	@echo "Cleanup up release folder"
	rm -rf release
	@echo "Cleanup completed !!"

compile:
	cargo build

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

build:
	@echo "Initializing clean build.."
	rm -rf release	
	cargo build --release
	@echo "Created Jotdown release build!"
	@echo "Creating release folder.."
	mkdir release		
	@echo "Packaging executable and release information.."
	cp ./target/release/jotdown release/jd
	cp ./*.md release/		
	@echo "Packaging completed !!"

dev: format lint test run
deploy: format lint test release
