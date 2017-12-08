#[macro_use] extern crate shell;

use shell::JobSpec;

fn main() {
    loop {
        cmd!("cargo test -- --test-threads=1").run().unwrap_or_default();
        cmd!("inotifywait -e close_write -r .").run().unwrap();
    }
}