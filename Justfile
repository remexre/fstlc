all: check build-debug doc test
	@just compile-example idid5
	@just compile-example plus
clean:
	cargo clean
watch TARGET="all":
	watchexec -cre lalrpop,rs,toml "just {{TARGET}}"

build: build-debug build-release
build-debug:
	cargo build
build-release:
	cargo build --release
check:
	cargo check --all
clippy:
	cargo clippy --all
doc:
	cargo doc --all
test:
	RUST_BACKTRACE=full cargo test --all -- --nocapture
	RUST_BACKTRACE=full cargo test --all --release -- --nocapture

outdated-deps:
	cargo outdated -R

compile-example NAME:
	@mkdir -p target/examples
	@just run examples/{{NAME}}.stlc -o target/examples/{{NAME}}.f
run +ARGS="":
	cargo run -- {{ARGS}}
