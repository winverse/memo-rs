use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Result as IoResult, Write},
    net::TcpStream,
};

use super::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // https://modoocode.com/334
    // dynamic disptch vs static dispatch
    // dynamic disptch -> 런타임에 정보가 바인딩
    // static dispatch -> 컴파일 단계에서 정보가 바인딩 되어 성능에 이점이 있음
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
