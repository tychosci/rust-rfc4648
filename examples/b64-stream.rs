//
// Usage:
//
//     b64-stream encode <infile> [outfile]
//     b64-stream decode <infile> [outfile]
//

extern mod encoding;

use io::Reader;
use encoding::BASE64;
use encoding::Base64Writer;
use encoding::Base64Reader;

fn main(args: ~[~str]) {
    let argc = args.len();

    if argc < 3 {
        return;
    }

    let writer = if argc > 3 {
        io::mk_file_writer(args[3], ~[io::Create, io::Truncate]).get()
    } else {
        io::stdout()
    };

    match args[1] {
        ~"encode" => encode(args[2], writer),
        ~"decode" => decode(args[2], writer),
        _         => return
    }
}

fn encode(filename: &str, writer: io::Writer) {
    let writer = Base64Writer(BASE64, writer);
    let reader = io::file_reader(filename.to_unique()).get();

    while !reader.eof() {
        let buf = reader.read_bytes(1024);
        writer.write(buf);
    }

    // FIXME Remove this line once we get drop intrinsics.
    writer.close();
}

fn decode(filename: &str, writer: io::Writer) {
    let reader = io::file_reader(filename.to_unique()).get();
    let reader = Base64Reader(BASE64, reader);

    while !reader.eof() {
        let buf = reader.read_bytes(1024);
        writer.write(buf);
    }
}
