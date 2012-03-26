use encoding;

import io::writer_util;
import encoding::base64;

fn main(args: [str]) {
    let mut args = args;
    let binary = vec::shift(args);
    let stdout = io::stdout();
    let stderr = io::stderr();

    if vec::len(args) < 2u {
        stderr.write_line(#fmt["Usage: %s <mode> <filename>", binary]);
        ret;
    }

    alt io::read_whole_file(args[1]) {
      result::ok(data) {
        let res = alt args[0] {
          "encode" { base64::encode(data) }
          "decode" { base64::decode(data) }
          _ { ret; }
        };
        stdout.write(res);
      }
      result::err(msg) {
        stderr.write_line(#fmt["Error: %s", msg]);
      }
    }
}
