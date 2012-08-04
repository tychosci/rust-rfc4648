use encoding;

import io::writer_util;
import encoding::codec;

fn main(args: ~[~str]) {
    let binary = copy args[0];
    let stdout = io::stdout();
    let stderr = io::stderr();

    if args.len() < 2u {
        stderr.write_line(fmt!{"Usage: %s <mode> <filename>", binary});
        return;
    }

    match io::read_whole_file(args[2]) {
        result::ok(data) => {
            let res = match args[1] {
                ~"encode" => data.encode(encoding::base64)
              , ~"decode" => data.decode(encoding::base64)
              , _         => return
            };
            stdout.write(res);
        }
        result::err(msg) => {
            stderr.write_line(#fmt["Error: %s", msg]);
        }
    }
}
