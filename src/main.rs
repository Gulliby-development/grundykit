use std::io::{Read, Write};

use grundykit::cli;
use grundykit::io::Io;

struct RealIo;

impl Io for RealIo {
    fn read_stdin(&self) -> Vec<u8> {
        let mut input = std::io::stdin();
        let mut bytes = Vec::new();
        let _ = input.read_to_end(&mut bytes);
        bytes
    }

    fn out(&self, text: &str) {
        let stdout = std::io::stdout();
        let mut output = stdout.lock();
        let _ = output.write_all(text.as_bytes());
        let _ = output.flush();
    }

    fn err(&self, text: &str) {
        let stderr = std::io::stderr();
        let mut output = stderr.lock();
        let _ = output.write_all(text.as_bytes());
        let _ = output.flush();
    }
}

fn main() {
    let args = std::env::args_os()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    std::process::exit(cli::run(&args, &RealIo));
}
