//
// Usage:
//
//     base64-stream encode <infile> [outfile]
//     base64-stream decode <infile> [outfile]
//

extern mod codec;

use io::Reader;
use io::ReaderUtil;
use path::Path;
use codec::binary::BASE64;
use codec::binary::Base64Writer;
use codec::binary::Base64Reader;

fn main() {
    let args = os::args();
    let argc = args.len();

    if argc < 3 {
        return;
    }

    let writer = if argc > 3 {
        io::mk_file_writer(&Path(args[3]), ~[io::Create, io::Truncate]).get()
    } else {
        io::stdout()
    };

    match args[1] {
        ~"encode" => encode(&Path(args[2]), writer),
        ~"decode" => decode(&Path(args[2]), writer),
        _         => return
    }
}

fn encode(filename: &Path, writer: io::Writer) {
    let writer = Base64Writer::new(BASE64, &writer);
    let reader = io::file_reader(filename).get();

    let mut buf = [0, ..1024];
    while !reader.eof() {
        let nread = reader.read(buf, buf.len());
        writer.write(vec::mut_view(buf, 0, nread));
    }
}

fn decode(filename: &Path, writer: io::Writer) {
    let reader = io::file_reader(filename).get();
    let reader = Base64Reader::new(BASE64, &reader);

    let mut buf = [0, ..1024];
    while !reader.eof() {
        let nread = reader.read(buf, buf.len());
        writer.write(vec::mut_view(buf, 0, nread));
    }
}