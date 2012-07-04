use encoding;

import io::writer_util;
import encoding::extensions;

fn main(args: ~[str]) {
    let binary = copy args[0];
    let stdout = io::stdout();
    let stderr = io::stderr();

    if vec::len(args) < 2u {
        stderr.write_line(#fmt["Usage: %s <mode> <filename>", binary]);
        ret;
    }

    alt io::read_whole_file(args[2]) {
        result::ok(data) {
            let res = alt args[1] {
                "encode" { data.encode(encoding::base64) }
                "decode" { data.decode(encoding::base64) }
                _ { ret; }
            };
            stdout.write(res);
        }
        result::err(msg) {
            stderr.write_line(#fmt["Error: %s", msg]);
        }
    }
}
