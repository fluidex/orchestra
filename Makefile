all: genpb gensw buildrs fmt lint

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

gensw:
	go mod tidy
	protoc -I. -Iproto/third_party/googleapis --swagger_out=logtostderr=true:. proto/exchange/matchengine.proto
	protoc -I. -Iproto/third_party/googleapis --swagger_out=logtostderr=true:. proto/rollup/rollup.proto

lint: lintrs

lintrs:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

runswui:
	docker-compose --file docker/docker-compose-swagger-ui.yaml --project-name fluidex-swagger-ui down --remove-orphans
	docker-compose --file docker/docker-compose-swagger-ui.yaml --project-name fluidex-swagger-ui up --force-recreate
