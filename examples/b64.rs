use encoding;

import io::writer_util;
import encoding::Codec;

fn main(args: ~[~str]) {
    let binary = copy args[0];
    let stdout = io::stdout();
    let stderr = io::stderr();

    if args.len() < 2u {
        stderr.write_line(fmt!("Usage: %s <mode> <filename>", binary));
        return;
    }

    match io::read_whole_file(args[2]) {
        result::ok(data) => match args[1] {
            ~"encode" => stdout.write(data.encode(encoding::Base64))
          , ~"decode" => stdout.write(data.decode(encoding::Base64))
          , _         => return
        }
        result::err(msg) => {
            stderr.write_line(fmt!("Error: %s", msg));
        }
    }
}
