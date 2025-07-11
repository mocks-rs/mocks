# E2E testing with runn

## About

> `runn` ( means "Run N". is pronounced /rʌ́n én/. ) is a package/tool for running operations following a scenario.

> Key features of runn are:
> - As a tool for scenario based testing.
> - As a test helper package for the Go language.
> - As a tool for workflow automation.
> - Support HTTP request, gRPC request, DB query, Chrome DevTools Protocol, and SSH/Local command execution
> - OpenAPI Document-like syntax for HTTP request testing.
> - Single binary = CI-Friendly.

See more: [k1LoW/runn: runn is a package/tool for running operations following a scenario.](https://github.com/k1LoW/runn)

## Testing

### Install

Homebrew

```shell
brew install k1LoW/tap/runn
```

Golang

```shell
go install github.com/k1LoW/runn/cmd/runn@latest
```

### Run E2E

```shell
# From the project root
cargo run -- run -H 127.0.0.1 ./runn-e2e/storage.runn.json
```

```shell
runn run ./runn-e2e/runbooks/test.yml --verbose
```
