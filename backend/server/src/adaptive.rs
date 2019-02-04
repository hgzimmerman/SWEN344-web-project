use hyper::{Method, Request};
use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use futures::future::join_all;
//use futures::future::join4;
use apply::Apply;
use hyper::Uri;

#[allow(dead_code)]
pub fn get_num_servers_up() -> impl Future<Item=usize, Error=()>  {
    let client = Client::new();

    let uri_1 = "http://129.21.208.2:3000/availability/1".parse().unwrap();
    let uri_2 = "http://129.21.208.2:3000/availability/2".parse().unwrap();
    let uri_3 = "http://129.21.208.2:3000/availability/3".parse().unwrap();
    let uri_4 = "http://129.21.208.2:3000/availability/4".parse().unwrap();


    let request_is_up = |uri: Uri| {
         client
             .get(uri)
             .and_then(|res| {
                 res.into_body().concat2()
             })
             .or_else(|_err| Ok(false)) // If the endpoint can't be reached, assume that the server isn't available.
             .map(|chunk| {
                 let v = chunk.to_vec();
                 let body = String::from_utf8_lossy(&v).to_string();
                 match body.as_str() {
                     "1" => true, // If the response is just "1" then the service is online.
                     _ => false
                 }
             })
    };

    let a_1 = request_is_up(uri_1);
    let a_2 = request_is_up(uri_2);
    let a_3= request_is_up(uri_3);
    let a_4 = request_is_up(uri_4);


    a_1.join4(a_2, a_3, a_4)
        .map(|(a, b, c, d)| {
            let mut acc = 0;
            if a {
                acc += 1;
            }
            if b {
                acc += 1;
            }
            if c {
                acc += 1;
            }
            if d {
                acc += 1;
            }
            acc
        })
}


#[test]
fn yeet() {
    assert!(get_num_servers_up().wait().is_ok())
}


