## setup

```sh
just dev-init
just dev-setup
dbmate up
cargo install clorinde
clorinde live -q ./crates/db/queries/ -d crates/clorinde
cargo build
```