//! (Self adaptive instructions section)[http://www.se.rit.edu/~swen-344/projects/selfadaptive/selfadaptive.html]

//use hyper::{Method, Request};
use hyper::Client;
use hyper::rt::{Future, Stream};
//use futures::future::join_all;
use hyper::Uri;
use hyper::Chunk;
use crate::error::Error;


/// Get the number of available servers.
#[allow(dead_code)]
pub fn get_num_servers_up() -> impl Future<Item=usize, Error=Error>  {
    let client = Client::new();

    let uri_1: Uri = "http://129.21.208.2:3000/availability/1".parse().unwrap();
    let uri_2: Uri = "http://129.21.208.2:3000/availability/2".parse().unwrap();
    let uri_3: Uri = "http://129.21.208.2:3000/availability/3".parse().unwrap();
    let uri_4: Uri = "http://129.21.208.2:3000/availability/4".parse().unwrap();


    let request_is_up = |uri: &Uri| {
         client
             .get(uri.clone())
             .and_then(|res| {
                 res.into_body().concat2()
             })
             .map(|chunk| {
                 let v = chunk.to_vec();
                 let body = String::from_utf8_lossy(&v).to_string();
                 match body.as_str() {
                     "1" => true, // If the response is just "1" then the service is online.
                     _ => false
                 }
             })
             .or_else(|_err| Ok(false)) // If the endpoint can't be reached, assume that the server isn't available.
    };

    let a_1 = request_is_up(&uri_1)
        .map_err(move |_: ()| Error::DependentConnectionFailed {url: uri_1.to_string() });
    let a_2 = request_is_up(&uri_2)
        .map_err(move |_: ()| Error::DependentConnectionFailed {url: uri_2.to_string() });
    let a_3= request_is_up(&uri_3)
        .map_err(move |_: ()| Error::DependentConnectionFailed {url: uri_3.to_string() });
    let a_4 = request_is_up(&uri_4)
        .map_err(move |_: ()| Error::DependentConnectionFailed {url: uri_4.to_string() });


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

/// Gets the "load" on the "servers".
#[allow(dead_code)]
pub fn get_load() -> impl Future<Item=usize, Error=Error>  {
    let uri: Uri = "http://129.21.208.2:3000/serverLoad".parse().unwrap();
    let client = Client::new();
     client
         .get(uri.clone())
         .and_then(|res| {
             res.into_body().concat2() // Await the whole body
         })
         .map_err(move |_| Error::DependentConnectionFailed {url: uri.to_string()})
         .and_then(|chunk: Chunk| {
             let v = chunk.to_vec();
             let body = String::from_utf8_lossy(&v).to_string();
             body.parse::<usize>().map_err(|_|crate::error::Error::InternalServerError)
         })
}

/// Per the assignment:
/// Each server can handle "10" load units
/// If the current server load exceeds what can be "served", then activate the dimmer (don't display the advertisement)
///
/// # Arguments
/// load_units - The current load across the "servers"
/// available_servers - The number (out of 4) of servers that are available.
pub fn should_serve_adds(load_units: usize, available_servers: usize) -> bool {
    const UNITS_PER_SERVER: usize = 10;
    let available_capacity = available_servers * UNITS_PER_SERVER;
    available_capacity > load_units
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn num_servers_up() {
        assert!(get_num_servers_up().wait().is_ok())
    }

    #[test]
    fn load() {
        get_load().wait();
    }
}




