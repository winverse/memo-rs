use super::method::{Method, MethodError}; // super는 paerent인 http 모듈을 가리킨다.
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        // buffer값 안에 무엇이 들어올지 예상할 수 없기 때문에 변환이 실패할 수 있음
        // 변환을 위한 rust 내장 모듈이 있음 std::convert
        // 실패하지 않는다면 std::convert::From 실패할 수도 있다면 std::convert::TryFrom
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?=name=abc&sort=1 HTTP/1.1\r\n...HEADERS
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }
        // 위의 match pattern도 가능하지만 아래 처럼 단순하게도 표현 가능
        // str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?

        // impl From<Utf8Error> for ParseError {} 이렇게 입력해서 사용하면 아래 처럼 사용 가능
        let request = str::from_utf8(buf)?;

        // match get_next_word(request) {
        //     Some((method, reuquest)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find("?") {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        unimplemented!()
    }
}

// 표준 라이브러리를 가져와서 이렇게 확장해서 맞춤형으로 사용가능!
// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }
// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }
// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }

// // 예시!
// fn trait_test(buf: &[u8]) {
//     let string = String::from("hello world");
//     string.encrypt();
//     buf.encrypt();
// }

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // Some(Method, Request header)
            return Some((&request[..index], &request[index + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {}
impl Error for ParseError {}
