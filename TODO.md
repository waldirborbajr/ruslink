```
cargo run
warning: use of deprecated type alias `std::panic::PanicInfo`: use `PanicHookInfo` instead
  --> src/app.rs:12:5
   |
12 |     human_panic::setup_panic!();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default
   = note: this warning originates in the macro `$crate::setup_panic` which comes from the expansion of the macro `human_panic::setup_panic` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: use of deprecated type alias `std::panic::PanicInfo`: use `PanicHookInfo` instead
  --> src/app.rs:12:5
   |
12 |     human_panic::setup_panic!();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this warning originates in the macro `$crate::setup_panic` which comes from the expansion of the macro `human_panic::setup_panic` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: method `is_destructive` is never used
  --> src/cli/config.rs:34:12
   |
32 | impl Config {
   | ----------- method in this implementation
33 |     /// Retorna se alguma operação destrutiva será realizada
34 |     pub fn is_destructive(&self) -> bool {
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `confirm` is never used
  --> src/utils/confirm.rs:40:8
   |
40 | pub fn confirm(message: &str, config: &Config) -> bool {
   |        ^^^^^^^

warning: function `info` is never used
  --> src/utils/output.rs:26:8
   |
26 | pub fn info(msg: &str) {
   |        ^^^^

warning: function `debug` is never used
  --> src/utils/output.rs:30:8

   |
30 | pub fn debug(msg: &str) {
   |        ^^^^^

warning: `ruslink` (bin "ruslink") generated 6 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/ruslink`
```
