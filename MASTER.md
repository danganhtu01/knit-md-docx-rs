# knit-md-docx-rs — master

> **last write-back: 2026-10-09T19:31:34+07** — R-1860: TODO.csv and TODO_LEDGER.csv moved to the fleet's C-19 header with a Details column, each live Task a short title with its context in Details; K-005 and K-006 retired as Superseded, answered by R-1580 and C-7 (R-1854); the task-file vocabulary below rewritten to match.

The owner's fork of [bokuweb/docx-rs](https://github.com/bokuweb/docx-rs), a `.docx` writer and
reader in Rust. The crate in [`docx-core/`](docx-core/) is published under the package name
**`knit-md-docx-rs`** and keeps the library name `docx_rs`, so consumers still write
`use docx_rs::*`. The fork adds native OMML (`m:oMath`) equations, run-level
superscript/subscript and paragraph borders. The public face of the library is
[`README.md`](README.md); this file is the map, and the only file here that carries a heartbeat.

## The suite

The owner calls the pair **knit-md-docx-rs**, and one assistant owns both repositories:

| Repository | What it is | How it depends |
| --- | --- | --- |
| `/srv/GitHub/knit-md-docx-rs` (this one, the hub) | the writer library, package `knit-md-docx-rs` | — |
| `/srv/GitHub/knit-md-docx` (entry point `MASTER.md`) | the Markdown-to-docx converter `rust_knit_md_docx` and its CLI `knit-md-docx` | path dependency on `../knit-md-docx-rs/docx-core` |

Consumers outside the pair, found from their code (first 2026-09-24, rechecked 2026-09-27):

- **ff-lc-app** — `rust_knit_md_docx` as a path dependency with `default-features = false`; its
  Dockerfile copies both repositories in as named build contexts, which is why each has a
  `.dockerignore`.
- **llm-skills** — the `deutsch-perfekt-glossieren` skill knits with the `knit-md-docx` binary.
- **os-config** — terminal-config installs the CLI on every machine from its tagged GitHub
  releases, pinned in its `pins.conf` (`KNIT_MD_DOCX_PIN`, with each asset's SHA-256). The
  release pipeline was built under K-003, now in [`TODO_LEDGER.csv`](TODO_LEDGER.csv).

## Where the state lives

| File | Role |
| --- | --- |
| [`TODO.csv`](TODO.csv) | the task file: open work only, one row per task |
| [`TODO_LEDGER.csv`](TODO_LEDGER.csv) | finished rows, moved whole under the same header; searched, never loaded |
| this file | the heartbeat and the map |
| `/srv/project-assistant/registry/knit-md-docx-rs.txt` | the registry entry the chief reads (outside the repository) |

**The task file's vocabulary.** Both files carry the fleet's one header (C-19 in project-assistant
`references/conventions.md`): `ID,Task,Status,Raised,Due,Waits on,Note,Assistant,Project,Subproject,Sub-subproject,Details`.
IDs are `K-` and three digits. Open states: `Open`, `Blocked`. A finished row takes `Done` or
`Superseded` and moves to the ledger in the same edit. `Task` is a short title (80 characters at
most); everything else about the task goes in `Details`, which becomes the task's page in Notion.
`Assistant` is `knit_md_docx_rs` and `Project` is `knit-md-docx-rs` on every row. A task for the
sibling repository is filed here, with `knit-md-docx` in its `Subproject` cell.

## Commands

```sh
make test            # cargo test -- --test-threads=1, the whole workspace
make lint            # clippy, warnings as errors
cargo test -p knit-md-docx-rs
```

The toolchain is pinned by [`rust-toolchain`](rust-toolchain). Snapshot tests use `insta`
(`cargo insta review` after an intended output change).

## Map

Root files:

| File | What it is |
| --- | --- |
| [`README.md`](README.md) | upstream's README with the fork notice; GitHub's front page |
| [`CHANGELOG.md`](CHANGELOG.md) | upstream's changelog; the fork's changes are in git history |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | upstream's contribution guide |
| [`LICENSE`](LICENSE) | upstream's MIT licence, unchanged; [`docx-core/LICENSE`](docx-core/LICENSE) is a byte copy for the crates.io package |
| [`SOURCE.txt`](SOURCE.txt) | where the fork came from: bokuweb/docx-rs at the fork point, its licence and the date taken; read it before changing upstream code |
| [`Cargo.toml`](Cargo.toml), [`Cargo.lock`](Cargo.lock) | the workspace: `docx-core`, `docx-wasm` |
| [`makefile`](makefile) | `test`, `lint`, and upstream's visual regression targets |
| [`rust-toolchain`](rust-toolchain), [`rustfmt.toml`](rustfmt.toml) | toolchain pin and format settings |
| [`renovate.json`](renovate.json) | upstream's dependency bot settings |
| [`.dockerignore`](.dockerignore) | keeps everything but `docx-core` out of ff-lc-app's build context |
| [`.gitignore`](.gitignore) | ignore rules |
| [`hello.docx`](hello.docx), [`hello.json`](hello.json), [`image.docx`](image.docx), [`logo.png`](logo.png) | upstream's sample outputs and logo |

GitHub and editor settings:

| File | What it is |
| --- | --- |
| [`.github/workflows/ci.yml`](.github/workflows/ci.yml) | upstream's CI: tests, build, wasm |
| [`.github/FUNDING.yml`](.github/FUNDING.yml) | upstream's funding link |
| [`.github/ISSUE_TEMPLATE/bug_report.md`](.github/ISSUE_TEMPLATE/bug_report.md), [`.github/ISSUE_TEMPLATE/feature_request.md`](.github/ISSUE_TEMPLATE/feature_request.md) | issue templates |
| [`.github/PULL_REQUEST_TEMPLATE.md`](.github/PULL_REQUEST_TEMPLATE.md) | pull request template |
| [`.vscode/settings.json`](.vscode/settings.json) | editor settings |

Directories, mapped as directories where they are upstream trees:

| Directory or file | What it is |
| --- | --- |
| [`docx-core/`](docx-core/README.md) | the crate `knit-md-docx-rs`: `src/` (documents, reader, xml builder), `tests/`, `examples/`, `benches/`; its [`README.md`](docx-core/README.md) is the crates.io readme, a byte copy of the root [`README.md`](README.md) kept because cargo packages only files inside `docx-core/`; edit both together |
| `docx-wasm/` | upstream's WebAssembly/JavaScript binding; not used by the pair; kept in its language under R-1580 (K-005 in [`TODO_LEDGER.csv`](TODO_LEDGER.csv)) |
| `fixtures/` | `.docx` fixtures the reader tests open |
| `docs/` | upstream's built demo page |
| [`images/cat.jpeg`](images/cat.jpeg), [`images/cat_min.jpg`](images/cat_min.jpg) | images the examples embed |
| [`output/.keep`](output/.keep), [`output/examples/.keep`](output/examples/.keep), [`output/js/.keep`](output/js/.keep) | keep the empty folders the examples write into |

**Carried graph warnings.** `check-graph` wants every tracked file listed one row each. The four
upstream trees above (`docx-core/`, `docx-wasm/`, `fixtures/`, `docs/`) hold over a thousand
source files and fixtures, which are mapped by directory here. Their `NO-INBOUND` and `NOT-IN-MAP`
warnings are carried by decision, not left by oversight: a per-file listing would be a
generated copy of `git ls-files` that goes stale with every upstream merge. The same goes for
`NO-OUTBOUND` on upstream's own documents, which point only at the web: [`README.md`](README.md),
[`CHANGELOG.md`](CHANGELOG.md), [`CONTRIBUTING.md`](CONTRIBUTING.md) and the three `.github` templates;
a map line added to them would be one more conflict at every upstream merge. Errors are not carried.
