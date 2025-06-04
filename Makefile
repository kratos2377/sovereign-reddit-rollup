check:
	SKIP_GUEST_BUILD=1 cargo check
	SKIP_GUEST_BUILD=1 cargo check --manifest-path crates/provers/risc0/guest-celestia/Cargo.toml
	SKIP_GUEST_BUILD=1 cargo check --manifest-path crates/provers/risc0/guest-mock/Cargo.toml
	SKIP_GUEST_BUILD=1 cargo check --manifest-path crates/provers/sp1/guest-celestia/Cargo.toml
	SKIP_GUEST_BUILD=1 cargo check --manifest-path crates/provers/sp1/guest-mock/Cargo.toml

lint:
	SKIP_GUEST_BUILD=1 cargo fmt --all -- --check
	SKIP_GUEST_BUILD=1 cargo check
	SKIP_GUEST_BUILD=1 cargo check --features celestia_da --features risc0 --no-default-features
	SKIP_GUEST_BUILD=1 cargo clippy
	SKIP_GUEST_BUILD=1 cargo clippy --features celestia_da --features risc0 --no-default-features
	SKIP_GUEST_BUILD=1 cargo clippy --features celestia_da --features sp1 --no-default-features
	zepter
	zepter
	zepter


install-risc0-toolchain:
	curl -L https://risczero.com/install | bash
	~/.risc0/bin/rzup install cargo-risczero 1.2.0
	cargo risczero install --version r0.1.81.0
	@echo "Risc0 toolchain version:"
	cargo +risc0 --version


install-sp1-toolchain:
	@echo "TOKEN"
	@echo "$$GITHUB_TOKEN" 
	curl -L https://raw.githubusercontent.com/succinctlabs/sp1/main/sp1up/install | bash
	~/.sp1/bin/sp1up --token "$$GITHUB_TOKEN" --version 3.4.0
	~/.sp1/bin/cargo-prove prove --version
	~/.sp1/bin/cargo-prove prove install-toolchain --token "$$GITHUB_TOKEN"
	@echo "SP1 toolchain version:"
	cargo +succinct --version

clean:
	@cargo clean
	@cargo clean --manifest-path crates/provers/risc0/guest-celestia/Cargo.toml
	@cargo clean --manifest-path crates/provers/risc0/guest-mock/Cargo.toml
	@cargo clean --manifest-path crates/provers/sp1/guest-celestia/Cargo.toml
	@cargo clean --manifest-path crates/provers/sp1/guest-mock/Cargo.toml
	rm -rf rollup-starter-data/
	rm -rf crates/rollup/mock_da.sqlite


# Use `--build-arg BUILD_MODE=release` for release builds
build-docker-mock-da:
	DOCKER_BUILDKIT=1 \
	docker build \
	--ssh default \
	-f ./docker/rollup/Dockerfile.mock \
	-t sov-rollup-starter:debug \
	.

run-docker-mock-da:
	mkdir -p docker/rollup/data
	mkdir -p docker/rollup/data/da
	mkdir -p docker/rollup/data/state
	docker run --rm -it \
    		-v $(CURDIR)/docker/rollup/data/da:/mnt/da \
    		-v $(CURDIR)/docker/rollup/data/state:/mnt/state \
    		-v $(CURDIR)/configs/mock/rollup-dockerized.toml:/app/config/rollup.toml \
    		-p 12346:12346 \
    		sov-rollup-starter:debug
