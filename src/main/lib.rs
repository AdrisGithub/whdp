pub mod method;
pub mod error;
pub mod version;
pub mod request;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use crate::request;

    use crate::request::Request;

    #[test]
    fn test_request() {
        let mut file = File::open("src/resources/request.txt").unwrap();
        let mut string = String::new();
        let _ = file.read_to_string(&mut string);
        let req = Request::try_from(string.as_str()).unwrap();
        println!("{}", req.get_body());
        println!("{}", req.get_method());
        println!("{}", req);
        println!("{:?}", req.get_headers().get("Host"));
        let req = Request::try_from(string.as_bytes()).unwrap();
    }
}