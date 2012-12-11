extern mod std;
extern mod codec;

use std::net::ip;
use std::net::tcp;
use std::uv_global_loop;

use io::{ReaderUtil, WriterUtil};
use tcp::{TcpErrData, TcpNewConnection, TcpSocket};
use task::{SingleThreaded, task};
use codec::binary::{BASE64, Base64Writer};

type KillChan = comm::Chan<Option<TcpErrData>>;
type ContChan = comm::Chan<()>;

fn main() {
    let iotask = uv_global_loop::get();

    let addr = "127.0.0.1";
    let port = 1337;

    let backlog = 128;
    let ip_addr = ip::get_addr(addr, iotask);
    let ip_addr = copy result::unwrap(ip_addr)[0];

    tcp::listen(ip_addr, port, backlog, iotask,
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
        Ok(socket) => {
            cont_ch.send(());
            encode(socket);
        }
        Err(err_data) => {
            kill_ch.send(Some(err_data));
            cont_ch.send(());
        }
    }
}

fn encode(socket: TcpSocket) {
    let socket = tcp::socket_buf(socket);
    let writer = Base64Writer::new(BASE64, &socket);
    let mut buf = [0, ..1024];
    while !socket.eof() {
        let nread = socket.read(buf, buf.len());
        writer.write(buf.view(0, nread));
    }
}
