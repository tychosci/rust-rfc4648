//
// Usage:
//
//     b64-stream encode <infile> [outfile]
//     b64-stream decode <infile> [outfile]
//

extern mod encoding;

use io::Reader;
use io::ReaderUtil;
use path::Path;
use encoding::BASE64;
use encoding::Base64Writer;
use encoding::Base64Reader;

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
    let writer = &Base64Writer(BASE64, writer);
    let reader = io::file_reader(filename).get();

    let mut buf = [mut 0, ..1024];
    while !reader.eof() {
        let nread = reader.read(buf, buf.len());
        writer.write(vec::mut_view(buf, 0, nread));
    }

    // FIXME Remove this line once we get Drop trait.
    writer.close();
}

fn decode(filename: &Path, writer: io::Writer) {
    let reader = io::file_reader(filename).get();
    let reader = &Base64Reader(BASE64, reader);

    let mut buf = [mut 0, ..1024];
    while !reader.eof() {
        let nread = reader.read(buf, buf.len());
        writer.write(vec::mut_view(buf, 0, nread));
    }
}
