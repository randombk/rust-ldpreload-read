# rust-ldpreload-read
LD_PRELOAD hook to intercept `read` calls to `glibc`, written in Rust with the assistance of https://github.com/geofft/redhook.

## Usage

Edit string constants `REDIRECT_ORIG_PATH` and `REDIRECT_TARGET_PATH` in `src/lib.rs`, then recompile with `cargo build`. When exeuting a program with `LD_PRELOAD` set to the output library, all reads to `REDIRECT_ORIG_PATH` will instead go to `REDIRECT_TARGET_PATH`.

## Example

Supposing that this library was compiled using `REDIRECT_ORIG_PATH="file1.txt"` and `REDIRECT_TARGET_PATH="file2.txt"`:

```sh
$ echo "File 1" > file1.txt
$ echo "File 2" > file2.txt
$ LD_PRELOAD=./libredirect_read.so cat file1.txt 
Redirected open("file1.txt") to open("file2.txt")
File 2

```
