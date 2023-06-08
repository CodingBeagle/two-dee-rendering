# Cargo Workspaces

A workspace is a set of packages sharing the same Cargo.lock and output directory.

The workspace has a single target directory that the compiled artifacts will be placed into.

This is because the crates in a workspace are meant to depend on each other.
