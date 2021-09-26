# Orchestra

Orchestra is a project of one `docker-compose` for full [fluiex-backend](https://github.com/fluidex/fluidex-backend).

It contains major features as below (for now):

* RPC protocol and generation for [dingir-exchange](https://github.com/fluidex/dingir-exchange) and [rollup-state-manager](https://github.com/fluidex/rollup-state-manager)
* `docker-compose` for full [fluiex-backend](https://github.com/fluidex/fluidex-backend)
* `docker-compose` for Swagger UI

## Prerequisite

* golang (for Swagger JSON generation)
* rust

## Make Commands

List the major make commands as below. Reference the comments in `Makefile` for all commands.

### RPC build and generation

User could run `make` directly for RPC generation if any protocol file `*.proto` is updated.
The RPC Rust code should be updated in folder `src/rpc`.

### Run `docker-compose` for full [fluiex-backend](https://github.com/fluidex/fluidex-backend)

User could run `make runbe` to start `docker-compose` for full [fluiex-backend](https://github.com/fluidex/fluidex-backend).
And `make stopbe` for stop.

### Run `docker-compose` for Swagger UI

User could run `make runswui` to start `docker-compose` for Swagger UI.
And `make stopswui` for stop.
