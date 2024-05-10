# Usage
1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
> [!TIP]
> Unix based operating systems can type the following command:
>```
>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --help
>```
2. Clone the repository and if successful enter the directory.
```
git clone https://github.com/charlesalaras/nine-men.git
cd nine-men
```
3. Build the program using caro and run the compiled binary. Note that you may require an internet connection to build the program.
```
cargo build -r
./target/release/nine-men
```
> [!TIP]
> You can alternatively enter `cargo run -r` to build and  run the program directly. If you need to pass arguments to the program, type `cargo run -r -- [OPTIONS]`

You can pass in arguments to the compiled program to alter the standard behavior. Passing the flag `--help` will display the following information:
```
Usage: nine-men [OPTIONS]

Options:
      --no-trace           Disable trace to stdout
  -l, --logfile <LOGFILE>  Path of file to write trace to
  -t, --time               Measure and output search time
  -h, --help               Print help
  -V, --version            Print version
```

> [!IMPORTANT]
> Note that flag combinations could cause different behavior:
> - Disabling trace and writing a log file will direct output to the file
> - Timing without diabling trace will present a warning to the user
