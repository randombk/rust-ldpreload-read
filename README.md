# rust-ldpreload-read
LD_PRELOAD hook to intercept `read` calls to `glibc`, written in Rust with the assistance of https://github.com/geofft/redhook.

## Usage

Edit string constants `REDIRECT_ORIG_PATH` and `REDIRECT_TARGET_PATH` in `src/lib.rs`, then recompile with `cargo build`. When exeuting a program with `LD_PRELOAD` set to the output library, all reads to `REDIRECT_ORIG_PATH` will instead go to `REDIRECT_TARGET_PATH`.
