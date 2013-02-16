extern mod std;
extern mod rfc4648;

use std::net::ip;
use std::net::tcp;
use std::uv_global_loop;
use std::cell::Cell;

use core::io::{ReaderUtil, WriterUtil};
use core::task::{SingleThreaded, task};
use std::net::tcp::{TcpErrData, TcpNewConnection, TcpSocket};
use rfc4648::base64::{BASE64_STD, Base64Writer};

type KillChan = pipes::SharedChan<Option<TcpErrData>>;
type ContChan = pipes::ChanOne<()>;

fn main() {
    let iotask = uv_global_loop::get();

    let addr = "127.0.0.1";
    let port = 1337;

    let backlog = 128;
    let ip_addr = ip::get_addr(addr, &iotask);
    let ip_addr = copy result::unwrap(ip_addr)[0];

    tcp::listen(ip_addr, port, backlog, &iotask,
                on_established, on_new_connection);
}

fn on_established(_kill_ch: KillChan) {
    io::println("running at 127.0.0.1:1337");
}

fn on_new_connection(conn: TcpNewConnection, kill_ch: KillChan) {
    let (cont_po, cont_ch) = pipes::oneshot();
    // NB: These cells can be removed once one-shot closure is landed.
    let kill_ch_cell = Cell(kill_ch);
    let cont_ch_cell = Cell(cont_ch);
    // Spawn new child which doesn't propagate errors to parent.
    do task().unlinked().sched_mode(SingleThreaded).spawn {
        let kill_ch = kill_ch_cell.take();
        let cont_ch = cont_ch_cell.take();
        accept(conn, kill_ch, cont_ch);
    }
    // Wait `tcp::accept(conn)' complete.
    cont_po.recv();
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
    let writer = Base64Writer::new(BASE64_STD, &socket);
    let mut buf = [0, ..1024];

    while !socket.eof() {
        let nread = socket.read(buf, buf.len());
        writer.write(buf.slice(0, nread));
    }
    writer.close();
}
