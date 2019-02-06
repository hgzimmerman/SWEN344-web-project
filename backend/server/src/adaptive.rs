//! (Self adaptive instructions section)[http://www.se.rit.edu/~swen-344/projects/selfadaptive/selfadaptive.html]

//use hyper::{Method, Request};
use hyper::Client;
use hyper::rt::{Future, Stream};
//use futures::future::join_all;
use hyper::Uri;
use hyper::Chunk;


/// Get the number of available servers.
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

/// Gets the "load" on the "servers".
#[allow(dead_code)]
pub fn get_load() -> impl Future<Item=usize, Error=crate::error::Error>  {
    let uri = "http://129.21.208.2:3000/serverLoad".parse().unwrap();
    let client = Client::new();
     client
         .get(uri)
         .and_then(|res| {
             res.into_body().concat2() // Await the whole body
         })
         .map_err(|_| crate::error::Error::InternalServerError)
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




