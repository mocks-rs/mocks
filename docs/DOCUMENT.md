# mocks docs

## Build for Homebrew

```shell
cargo build --release
cd target/release
tar -czf mocks-X.X.X-x86_64-apple-darwin.tar.gz mocks
shasum -a 256 mocks-X.X.X-x86_64-apple-darwin.tar.gz
```

## Check MSRV

```shell
cargo install cargo-msrv --locked
cargo msrv find
```

For more details: [cargo-msrv](https://github.com/foresterre/cargo-msrv)
