# aoheyra-fuzz

Central fuzz repository for the aoheyra organisation. All fuzz targets and
their seeds live here; the code repositories contain no `fuzz/` directory and
no fuzz workflow.

## Schedule

Every Sunday at 00:00 IST (Saturday 18:30 UTC, GitHub cron `30 18 * * 6`) this
repository runs one job per fuzz crate. Each job splits 5,400 seconds of
libFuzzer time across its targets, at least 10 minutes per target, and uploads
crashes plus new corpus entries as artifacts when a run fails. A separate job
validates the documentation-only `aoheyra` repository.

## Crates

| Crate | Repository | Targets |
|---|---|---|
| `fuzz/ao-mesh-fuzz` | aoMesh | `packet_decode`, `engine_input`, `sync_frames` |
| `fuzz/ao-network-fuzz` | aoNetwork | `beacon_decode`, `frame_decode`, `reassembler_push` |
| `fuzz/ao-idents-fuzz` | aoIdents | `seal_decode_open`, `card_decode`, `signed_decode`, `group_decode`, `prekey_decode`, `handshake_read` |

Each crate pins the sibling revision it fuzzes in `Cargo.toml` and commits a
`Cargo.lock`. When a code repository changes its wire format or engine input
surface, update the pinned revision and the target in the same PR.

## Adding or retiring a target

1. Add or remove the target source under the correct `fuzz/<crate>/fuzz_targets/`.
2. Add or remove the matching `[[bin]]` entry in that crate's `Cargo.toml`.
3. Add seeds under `fuzz/<crate>/seeds/<target>/` when a known-good input exists.
4. Update the table above and the docs issue in `aoheyra/aoheyra-docs`.
5. Run `cargo +nightly fuzz run <target> -- -runs=10000` locally before opening the PR.

No repository secret is required: the scheduler runs entirely in this repository.
