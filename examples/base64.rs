extern mod rfc4648;

use rfc4648::FromBase64;
use rfc4648::ToBase64;

use core::path::Path;

fn main() {
    let args = os::args();

    let binary = &args[0];
    let stdout = io::stdout();
    let stderr = io::stderr();

    if args.len() < 2u {
        stderr.write_line(fmt!("Usage: %s <mode> <filename>", *binary));
        return;
    }

    match io::read_whole_file(&Path(args[2])) {
        Err(msg) => {
            stderr.write_line(fmt!("Error: %s", msg));
        }
        Ok(data) => match args[1] {
            ~"encode" => stdout.write(data.to_base64()),
            ~"decode" => stdout.write(data.from_base64()),
            _         => ()
        }
    }
}
