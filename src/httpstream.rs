// NOT USED
//extern crate hyper;
//
//use hyper::Client;
//use std::io::Read;
//use std::io::Result;
//
//fn stream(url: &str, callback: &Fn(&[u8], usize)) {
//    let client = Client::new();
//    let mut response = client.get(url).send().unwrap();
//
//    assert_eq!(response.status, hyper::Ok);
//
//    // Buffer len 256000
//    let mut buf: [u8; 256000] = [0; 256000];
//
//    loop {
//        let result: Result<usize> = response.read(&mut buf[..]);
//
//        if result.is_err() {
//            println!("Error!");
//            break;
//        }
//
//        callback(&buf, result.ok().unwrap());
//    }
//}
//
