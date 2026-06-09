# Release procedure

1. Update the version in Cargo.toml.
2. Update the "Getting started" instruction in src/lib.rs.
3. Run `cargo readme > README.md`.
4. Commit and push changes.
5. Create and push a `vX.Y.Z` tag for the new version. The tag version must match the version in Cargo.toml.
6. Approve the `release` environment deployment in GitHub Actions. The new version will be published on crates.io after approval.
7. Create a release on GitHub for the tag.
