use std::io::{BufReader, Read};
use std::{fs, sync::Arc};

use self::network::{Acceper, NetIo, NetWorker};
pub struct StockAdv1 {}
impl StockAdv1 {
    pub fn getContent<'a>(rpath: &'a str) -> Option<Arc<&[u8]>> {
        let mut f = fs::File::open(rpath).unwrap();
        let mut buff: &[u8] = &[0u8; 1 << 10];
        let buff2 = &buff[..];
        return None;
    }
    pub fn listen(_addr: String, _port: u16) {
        let mut nw = NetWorker::new(_addr, _port);
        nw.connect();
        let mut conn = nw.accept();
        let greet = "hello new connect!";
        unsafe {
            conn.send(greet.to_owned().as_bytes_mut());
        }
        let mut buff = [0u8; 1 << 10];
        let ref mut buffptr = buff[..];
        conn.recv(buffptr);
        // unsafe {
        println!("accept\n{}", String::from_utf8_lossy(&buffptr));
        // }
    }
}
mod network {
    use std::{
        borrow::Cow,
        io::{Read, Write},
        net::SocketAddr,
        net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
        str::FromStr,
    };

    pub trait NetIo {
        fn connect(&mut self);
        fn close(&self);
        fn send(&mut self, buff: &mut [u8]);
        fn recv<'a>(&mut self, buff: &'a mut [u8]);
    }
    pub trait Acceper {
        fn accept(&self) -> Box<dyn NetIo>;
    }
    #[allow(dead_code)]
    pub struct NetWorker {
        _port: u16,
        _addr: String,
        _listener: Option<TcpListener>,
    }
    pub struct NetConn {
        _addr: SocketAddr,
        _conn: TcpStream,
    }
    impl NetWorker {
        pub fn new(_addr: String, _port: u16) -> Self {
            return Self {
                _port: _port,
                _addr: _addr,
                _listener: None,
            };
        }
    }
    impl NetIo for NetWorker {
        fn connect(&mut self) {
            let addr = Ipv4Addr::from_str(self._addr.as_str());
            let sockv4 = SocketAddrV4::new(addr.unwrap(), self._port);
            self._listener = Some(TcpListener::bind(sockv4).unwrap());
        }

        fn close(&self) {
            println!("tcp listener {}:{} closed", self._addr, self._port);
        }

        fn send(&mut self, buff: &mut [u8]) {
            panic!("not implement");
        }

        fn recv<'a>(&mut self, buff: &'a mut [u8]) {
            panic!("not implement");
        }
    }
    impl Acceper for NetWorker {
        fn accept(&self) -> Box<dyn NetIo> {
            match &self._listener {
                None => panic!("not init listener"),
                Some(listener) => {
                    let conninfo = listener.accept();
                    let info = conninfo.unwrap();
                    let result: Box<dyn NetIo> = Box::new(NetConn {
                        _addr: info.1,
                        _conn: info.0,
                    });
                    return result;
                }
            }
        }
    }
    impl NetIo for NetConn {
        fn connect(&mut self) {
            panic!("not implement");
        }

        fn close(&self) {
            let _ = self._conn.shutdown(std::net::Shutdown::Both);
        }

        fn send(&mut self, buff: &mut [u8]) {
            let _ = self._conn.write(buff);
        }

        fn recv<'a>(&mut self, buff: &'a mut [u8]) {
            self._conn.read(buff).unwrap();
        }
    }
    impl Drop for NetWorker {
        fn drop(&mut self) {
            self.close();
        }
    }
    impl Drop for NetConn {
        fn drop(&mut self) {
            self.close();
        }
    }
}
