# Bs2: a RiscV ZKVM for ckb

## Workflow

### Generating program trace

Run sample program and dump trace to file `trace.json`.

```
cd trace_dumper
cargo run --bin=ckb-debugger -- --mode=trace_dump --tx-file=ckb-debugger-api/tests/programs/sample_data1.json --script-group-type=type --cell-type=output --cell-index=0 --trace-file=../trace.json
```


### Prove
```
cargo run --bin cli -- prove --trace trace.json
```

### Build the verifier

```
rustup target add riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release
```

### Run the verifier

Note that currently the verifying key, kzg parameters and proof are hard-coded in the code.
In the future, we will pass program hash, hash for ckb transactions, verifying key, kzg parameter and proof to the verifier via ckb syscall, and the verifier can verify the proof on-chain.

```
cd trace_dumper
cargo run --release --bin=ckb-debugger -- --mode=fast --max-cycles 9999999999 --bin ../target/riscv64imac-unknown-none-elf/debug/bs2_verifier
```