# ctx

The ContextDrive client ([contextdrive.io](https://contextdrive.io)). This repository is `contextdrive/ctx`.

`ctx` runs on the caller's machine. `ctx login` stores a credential. `ctx ingest` walks a local tree, hashes it, asks `server` for a file row and an upload grant, writes the bytes to the endpoint on that grant, and commits. `ctx status` reads the job. Parsing, embeddings, SQL, and policy stay in `contextdrive/server`. The client does not choose a bucket. `ctx` is Rust, a native binary, and it does not link DuckDB or Tika.

Product vision: private repo `contextdrive/vision` on `main`. How to read it, and the command boundaries, are in [AGENTS.md](AGENTS.md).

The crate in this tree is the template. The three commands parse, and the work they call is not built yet. `ctx login` is the open issue.

## Build

```shell
cargo test
cargo run -- --help
```

The binary is `target/debug/ctx`.
