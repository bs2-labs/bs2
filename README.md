## Bs2

### Generate program trace

Run sample program and dump trace to file `trace.json`.

```
cargo run --manifest-path=$PWD/trace_dumper/Cargo.toml --bin=ckb-debugger -- --mode=trace_dump --tx-file=$PWD/trace_dumper/ckb-debugger-api/tests/programs/sample_data1.json --script-group-type=type --cell-type=output --cell-index=0 --trace-file=trace.json
```


### Prove
```
cargo run -- prove
```

