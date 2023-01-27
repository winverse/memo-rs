use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // a: [u8; 8] -> 8비트로 구성된 8개 크기의 Array(벡터와 배열의 차이를 알면 왜 이렇게 해야하는지 알 수 있음)
                    // [0; 1024] -> 0으로 1024개의 공간을 채움; 0 -> 8비트, * 1024 => 1킬로바이트
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            //[..] 의미 -> 전체 Array가 byte slice에 담기게 되어  try_from의 파라미터 &[u8] 요구 타입이 맞음
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => {
                            println!("Failed to establish a connection : {}", e);
                        }
                    }
                }
                Err(e) => println!("Failed to establish a connection {}", e),
            }
        }
    }
}
