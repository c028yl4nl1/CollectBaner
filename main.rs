use std::env::args;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;
use std::process::exit;
use native_tls::TlsConnector;

struct Get {
    Host: String,
    Port: String,
}

impl Get {
    fn new(Host: String, Port: String) -> Get {
        Get { Host, Port }
    }

    fn http_collect(&self) -> Option<String> {
        if let Ok(mut cn) = TcpStream::connect(format!("{}:{}", self.Host, self.Port)) {
            let mut buffer_recv = [0; 5000];
            if let Ok(_) = cn.write(b"HEAD / HTTP/1.0\r\n\r\n") {
                let result = cn.set_read_timeout(Some(Duration::from_secs(5)));
                match result {
                    Ok(_) => {
                        match cn.read(&mut buffer_recv) {
                            Ok(_) => {
                                return Some(String::from_utf8_lossy(&buffer_recv).to_string());
                            },
                            Err(_) => {
                                println!("Host não respondeu");
                                return None;
                            },
                        }
                    },
                    Err(_) => {
                        return None;
                    },
                }
            }
        }
        println!("Host Off");
        None
    }

    fn tls_ssl_connect_https(&self) {
        let connector = TlsConnector::new();
        if let Ok(connector) = connector {
            let port_https = 443;
            if let Ok(connect) = TcpStream::connect(format!("{}:{}", self.Host, port_https)) {
                if let Ok(mut https_tcp_tls) = connector.connect(&self.Host, connect) {
                    let mut buffer_recv = [0; 4000];
                    if let Ok(_) = https_tcp_tls.write_all(b"HEAD / HTTP/1.0\r\n\r\n") {
                        match https_tcp_tls.read(&mut buffer_recv) {
                            Ok(_) => {
                                println!("{}", String::from_utf8_lossy(&buffer_recv));
                            },
                            Err(_) => {
                                println!("Servidor sem resposta");
                                exit(0);
                            },
                        }
                    }
                } else {
                    println!("Host sem SSL");
                    exit(2);
                }
            } else {
                println!("Host HTTPS desligado");
                exit(1);
            }
        } else {
            println!("Erro TLS");
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("Usage: ./run Host Port");
        exit(0x0100);
    }

    let host = &args[1];
    let port = &args[2];

    let getter = Get::new(host.clone(), port.clone());
    getter.http_collect();
    getter.tls_ssl_connect_https();
}
