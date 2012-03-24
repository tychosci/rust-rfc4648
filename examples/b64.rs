use encoding;

import io::writer_util;
import encoding::base64;

fn main(args: [str]) {
    let mut args = args;
    let binary = vec::shift(args);
    let stdout = io::stdout(), stderr = io::stderr();
    let b64 = base64::mk();
    if vec::len(args) < 2u {
        stderr.write_line(#fmt["Usage: %s <mode> <filename>", binary]);
        ret;
    }
    alt io::read_whole_file(args[1]) {
      result::ok(data) {
        let res = alt args[0] {
          "encode" { b64.encode_bytes(data) }
          "decode" { b64.decode_bytes(data) }
          _ { ret; }
        };
        stdout.write(res);
      }
      result::err(msg) {
        stderr.write_line(#fmt["Error: %s", msg]);
      }
    }
}
