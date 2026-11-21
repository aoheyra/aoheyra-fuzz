# aoheyra-fuzz

Central weekly fuzzing scheduler for the aoheyra organisation.

## Schedule

Every Sunday at 00:00 IST (Saturday 18:30 UTC, GitHub cron `30 18 * * 6`) this
repository starts the 90-minute fuzz workflow in `aoMesh`, `aoNetwork` and
`aoIdents`, and validates the documentation-only `aoheyra` repository.

Each code repository splits 5,400 seconds of libFuzzer time across its own
targets and uploads crashes plus new corpus entries as artifacts when a run
fails. The target list stays in each repository so it owns its fuzz surface.

## Required secret

`FUZZ_DISPATCH_TOKEN` must be a repository secret with `Actions: write` on the
three code repositories. The default `GITHUB_TOKEN` is scoped to this repository
and cannot dispatch workflows elsewhere.

## Repositories

| Repository | Weekly job |
|---|---|
| aoMesh | 5,400 seconds across `fuzz/fuzz_targets/*.rs` |
| aoNetwork | 5,400 seconds across `fuzz/fuzz_targets/*.rs` |
| aoIdents | 5,400 seconds across `fuzz/fuzz_targets/*.rs` |
| aoheyra | documentation link and workflow syntax validation (docs-only repo) |
