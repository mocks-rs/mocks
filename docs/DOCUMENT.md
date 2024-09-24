# mocks docs

## Build

```shell
cargo build --release
cd target/release
tar -czf mocks-X.X.X-x86_64-apple-darwin.tar.gz mocks
shasum -a 256 mocks-X.X.X-x86_64-apple-darwin.tar.gz
```
