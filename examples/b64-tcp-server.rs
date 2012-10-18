extern mod std;
extern mod encoding;

use std::net::ip;
use std::net::tcp;
use std::uv_global_loop;

use encoding::{BASE64, Base64Writer};
use io::{ReaderUtil, WriterUtil};
use tcp::{TcpErrData, TcpNewConnection, TcpSocket};
use task::{SingleThreaded, task};

type KillChan = comm::Chan<Option<TcpErrData>>;
type ContChan = comm::Chan<()>;

fn main() {
    let iotask = uv_global_loop::get();

    let addr = "127.0.0.1";
    let port = 1337;

    let backlog = 128;
    let ip_addr = ip::get_addr(addr, iotask);
    let ip_addr = copy result::unwrap(move ip_addr)[0];

    tcp::listen(move ip_addr, port, backlog, iotask,
        |kill_ch| on_established(kill_ch),
        |conn, kill_ch| on_new_connection(conn, kill_ch));
}

fn on_established(_kill_ch: KillChan) {
    io::println("running at 127.0.0.1:1337");
}

fn on_new_connection(conn: TcpNewConnection, kill_ch: KillChan) {
    do comm::listen |cont_ch| {
        // Spawn new child which doesn't propagate errors to parent.
        do task().unlinked().sched_mode(SingleThreaded).spawn {
            accept(conn, kill_ch, cont_ch);
        }
        // Wait `tcp::accept(conn)' complete.
        cont_ch.recv();
    }
}

fn accept(conn: TcpNewConnection, kill_ch: KillChan, cont_ch: ContChan) {
    match tcp::accept(conn) {
        Ok(move socket) => {
            cont_ch.send(());
            encode(move socket);
        }
        Err(move err_data) => {
            kill_ch.send(Some(move err_data));
            cont_ch.send(());
        }
    }
}

fn encode(socket: TcpSocket) {
    let socket = tcp::socket_buf(move socket);

    let encoded_bytes = io::with_bytes_writer(|writer| {
        let writer = Base64Writer(BASE64, move writer);
        let mut buf = [mut 0, ..1024];
        loop {
            let nread = socket.read(buf, buf.len());
            if nread == 0 { break; }
            writer.write(buf.view(0, nread));
        }
        // FIXME: Remove this line once we get Drop trait.
        writer.close();
    });

    socket.write(encoded_bytes);
}
