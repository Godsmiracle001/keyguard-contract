include .env

CONTRACT_NAME=keyguard_contract
WASM_PATH=target/wasm32-unknown-unknown/release/$(CONTRACT_NAME).wasm

build:
	cargo build --target wasm32-unknown-unknown --release

fmt:
	cargo fmt

test:
	cargo test

deploy: build
	@soroban contract deploy \
		--wasm $(WASM_PATH) \
		--source $(SOROBAN_ACCOUNT) \
		--rpc-url $(SOROBAN_RPC_URL) \
		--network-passphrase $(SOROBAN_NETWORK_PASSPHRASE) \
		| tee contract-id.txt

clean:
	cargo clean