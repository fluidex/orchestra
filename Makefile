all: genpb buildrs fmt lint

buildrs:
	cargo build

fmt: fmtproto fmtrs

fmtproto:
	clang-format -i proto/exchange/matchengine.proto proto/rollup/rollup.proto

fmtrs:
	cargo fmt --all

genpb:
	cd proto && protoc -Ithird_party/googleapis -I. --include_imports --include_source_info --descriptor_set_out=exchange/matchengine.pb exchange/matchengine.proto
	cd proto && protoc -Ithird_party/googleapis -I. --include_imports --include_source_info --descriptor_set_out=rollup/rollup.pb rollup/rollup.proto

lint: lintrs

lintrs:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
