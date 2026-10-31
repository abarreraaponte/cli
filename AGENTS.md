# ctx

This repository is `ctx`, the ContextDrive client. It runs on the caller's machine. v1 commands are `ctx login`, `ctx ingest`, and `ctx status`. `ctx` is Rust, a native binary, and it does not link DuckDB or Tika. The crate is the template: the commands parse, and the work they call is not built yet.

The client walks, hashes, and uploads. It does not parse, embed, run SQL, or decide policy. It asks `server` for a file row and a short-lived upload grant, writes bytes to the endpoint on that grant, and commits. Embedding and parse run in `server` after the commit. This client does not accept a customer bucket.

## Repositories

Eight repositories. The same list is `ARCHITECTURE.md` section 10.

| Repo | Purpose |
| --- | --- |
| `vision` | Product vision, architecture, decisions, and brand. Markdown and assets only. |
| `web` | `contextdrive.io`. The public site and the docs. No auth, no database, no product records, and no drive UI. |
| `console` | `console.contextdrive.io`. The drive UI. A client-only SPA. |
| `server` | `api.contextdrive.io`. Java. Product records, policy, catalog, files, and audit. |
| `id` | `id.contextdrive.io`. Identity. Passwords, sessions, and tokens. No product records. |
| `ctx` | The client on the caller's machine. Rust. |
| `devops` | Bootstrap and deploy for a location. Terraform and Kamal. The server still creates each organization bucket. |
| `packages` | npm packages a function imports. `@contextdrive/drive` calls the API for files and a `.ctxdb`. A connector speaks to one outside system. Neither owns the allow. |

## Vision repository

Product vision, architecture, and decisions live in the private repository `contextdrive/vision`, on branch `main`. Read `PRODUCT_VISION.md` and `ARCHITECTURE.md` there before designing or implementing. This repository does not keep a product vision. Do not add one. The upload contract is `ARCHITECTURE.md`: a pending file row, then an upload grant for that key, then a commit. Customer-owned buckets are out. If a note or an issue in this repository disagrees with the vision repository on product behavior, the vision repository wins and that note or issue is updated. When the vision repository is what needs to change, edit it on `main` after the user agrees.

Reach `contextdrive/vision` in this order:

1. Local sibling checkout. From this repository root, `../vision` is the vision repository when the repos share a parent folder. Use that checkout only when it exists and `git -C ../vision rev-parse --abbrev-ref HEAD` prints `main`. Any other branch is not the source of truth.
2. Remote `main`, with the `gh` CLI. The logged-in token is the credential. The repository is private.

```shell
gh api repos/contextdrive/vision/contents/PRODUCT_VISION.md?ref=main --jq .content | base64 -d
gh api repos/contextdrive/vision/contents/ARCHITECTURE.md?ref=main --jq .content | base64 -d
```

`gh repo clone contextdrive/vision` and checking out `main` is the same remote read.

## Local instructions

Read `LOCAL.md` at this repository root when it exists. The file is optional and gitignored. It instructs agents about this machine only: local paths, local tools, and workflows that stay on this computer. Follow it for work in this checkout. Another checkout has its own file, or none. Create `LOCAL.md` only when asked. Leave its contents out of committed files, issues, and pull requests. Product behavior stays in `contextdrive/vision` on `main`. That repository’s `AGENTS.md` records the convention.

## Layout

`cli` parses arguments. `api` calls `server`. `config` names the credential file under the user config directory. A command calls those modules. It does not own the walk, the hash, or the upload. Debug output redacts the bearer token. Each GitHub issue in this repository is one command. Do not take on server work (storage, parsers, workers, policy) in a `ctx` change.
