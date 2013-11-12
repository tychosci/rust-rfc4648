extern mod rfc4648;

use std::os;
use std::rt::io;
use std::rt::io::{Reader, Writer};
use std::rt::io::File;

use rfc4648::base64;

fn main() {
    let args = os::args();

    let binary = &args[0];
    let mut stderr = io::stderr();
    let mut stdout = io::stdout();

    if args.len() < 2 {
        let usage = format!("Usage: {} <mode> <filename>\n", *binary);
        stderr.write(usage.into_bytes());
        stderr.flush();
        return;
    }

    let path = Path::new(args[2].clone());
    let data = File::open(&path).read_to_end();

    match args[1] {
        ~"encode" => stdout.write(base64::Standard.encode(data)),
        ~"decode" => stdout.write(base64::Standard.decode(data)),
        _ => ()
    }
}
