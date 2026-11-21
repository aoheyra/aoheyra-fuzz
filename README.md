# aoheyra-fuzz

Shared weekly fuzzing for the aoheyra repositories.

## Schedule

Every Sunday at 00:00 IST (Saturday 18:30 UTC, GitHub cron `30 18 * * 6`), the
code repositories call the reusable workflow in this repo. Each repository gets
a combined 90 minutes of libFuzzer execution, split evenly across its targets.
The workflow allows a further ten minutes for checkout, nightly toolchain and
`cargo-fuzz` installation.

| Repository | Fuzz targets |
|---|---|
| aoMesh | `packet_decode`, `engine_input`, `sync_frames` |
| aoNetwork | all `fuzz/fuzz_targets/*.rs` targets |
| aoIdents | all `fuzz/fuzz_targets/*.rs` targets |

The `aoheyra` application repo currently contains documentation only; it has no
Rust crate or fuzz targets yet and is not dispatched.

## Reusable workflow

`.github/workflows/fuzz-runner.yml` accepts a newline-separated list of target
names and runs them from the caller's checkout. Crashes and new corpus entries
are uploaded as artifacts when a run fails. Keep target lists in the caller
workflows so each repo owns its own fuzz surface.
