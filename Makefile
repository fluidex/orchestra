# Default make command
build: genpb gensw buildrs fmt lint

# Build Rust code
buildrs:
	cargo build

# Format all
fmt: fmtproto fmtrs

# Format proto files
fmtproto:
	clang-format -i proto/exchange/matchengine.proto proto/rollup/rollup.proto

# Format Rust code
fmtrs:
	cargo fmt --all

# Generate PB files
genpb:
	cd proto && protoc -Ithird_party/googleapis -I. --include_imports --include_source_info --descriptor_set_out=exchange/matchengine.pb exchange/matchengine.proto
	cd proto && protoc -Ithird_party/googleapis -I. --include_imports --include_source_info --descriptor_set_out=rollup/rollup.pb rollup/rollup.proto

# Generate Swagger JSON files
gensw: goinstall
	protoc -I. -Iproto/third_party/googleapis --openapiv2_out . --openapiv2_opt logtostderr=true proto/exchange/matchengine.proto
	protoc -I. -Iproto/third_party/googleapis --openapiv2_out . --openapiv2_opt logtostderr=true proto/rollup/rollup.proto

# Install go dependencies
goinstall:
	go mod tidy
	bash go-install.sh

# Lint all
lint: lintrs

# Lint Rust code
lintrs:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

# Start docker-compose for fluidex-backend
runbe: stopbe
	docker-compose --file docker/docker-compose.yaml --project-name fluidex-backend up --force-recreate # --detach

# Start docker-compose for Swagger UI
runswui: stopswui
	docker-compose --file docker/docker-compose-swagger-ui.yaml --project-name fluidex-swagger-ui up --force-recreate # --detach

# Stop docker-compose for fluidex-backend
stopbe:
	docker-compose --file docker/docker-compose.yaml --project-name fluidex-backend down --remove-orphans

# Stop docker-compose for Swagger UI
stopswui:
	docker-compose --file docker/docker-compose-swagger-ui.yaml --project-name fluidex-swagger-ui down --remove-orphans
