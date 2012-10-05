extern mod encoding;

use io::Writer;
use io::WriterUtil;
use encoding::Codec;
use path::Path;

fn main() {
    let args = os::args();

    let binary = copy args[0];
    let stdout = io::stdout();
    let stderr = io::stderr();

    if args.len() < 2u {
        stderr.write_line(fmt!("Usage: %s <mode> <filename>", binary));
        return;
    }

    match io::read_whole_file(&Path(args[2])) {
        Ok(move data) => match args[1] {
            ~"encode" => stdout.write(data.encode(encoding::Base64)),
            ~"decode" => stdout.write(data.decode(encoding::Base64)),
            _         => return
        },
        Err(ref msg) => {
            stderr.write_line(fmt!("Error: %s", *msg));
        }
    }
}
