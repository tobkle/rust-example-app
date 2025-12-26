## setup

```sh
just dev-init
just dev-setup
dbmate up
cargo install clorinde
clorinde live -q ./crates/db/queries/ -d crates/clorinde
cargo build

earthly -P --build-arg RUST_TARGET=aarch64-unknown-linux-musl --build-arg DBMATE_ARCH=arm64 +build-cache
```