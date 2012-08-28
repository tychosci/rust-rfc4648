extern mod encoding;

use io::WriterUtil;
use encoding::Codec;
use path2::Path;

fn main(args: ~[~str]) {
    let binary = copy args[0];
    let stdout = io::stdout();
    let stderr = io::stderr();

    if args.len() < 2u {
        stderr.write_line(fmt!("Usage: %s <mode> <filename>", binary));
        return;
    }

    match io::read_whole_file(&Path(args[2])) {
        result::Ok(data) => match args[1] {
            ~"encode" => stdout.write(data.encode(encoding::Base64)),
            ~"decode" => stdout.write(data.decode(encoding::Base64)),
            _         => return
        },
        result::Err(msg) => {
            stderr.write_line(fmt!("Error: %s", msg));
        }
    }
}
