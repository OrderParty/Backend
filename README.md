# Order Party

## Setup
### Install the [Rust toolchain](https://www.rust-lang.org/tools/install)

### Set up Diesel (with SQLite3)
#### Follow [these steps](#sqlite3-windows-setup-source) if you are using **Windows**, otherwise go to next step.
#### Diesel-CLI setup
```sh
cargo install diesel_cli --no-default-features --features sqlite
```
### Open project folder
```sh
cd {pathToProjectFolder}/OrderParty-Backend
```
### Run diesel database setup
```sh
diesel database setup
```
### Run Application
```sh
cargo run
```
### DONE


## SQLite3 Windows setup ([source](https://github.com/diesel-rs/diesel/issues/487#issuecomment-415752856))
* download the precompiled Windows binaries from https://sqlite.org/download.html
* extract them to a folder (i.e. C:\sqlite64)
* run a cmd terminal with the 64bit msvc toolchain in the path: "Win + R" and execute %comspec% /K "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" amd64
* cd C:\sqlite64 and run lib /def:sqlite3.def /out:sqlite3.lib
* add C:\sqlite64 to PATH
* create an environment variable SQLITE3_LIB_DIR also pointing to C:\sqlite64
* restart terminal and run cargo install diesel_cli --no-default-features --features sqlite


> Hint: The path to vcvarsall.bat might not be the same on your machine - just search for "vcvarsall" in the explorer and you will find it.