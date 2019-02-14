//! [Self adaptive instructions section][http://www.se.rit.edu/~swen-344/projects/selfadaptive/selfadaptive.html]

use crate::error::Error;
use hyper::{
    rt::{Future, Stream},
    Chunk, Client, Uri,
};

/// The fictional load encountered by the servers.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Load(pub u32);
/// The fictional number of servers currently available.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NumServers(pub u32);

/// Get the number of available servers.
///
/// # Return
/// A Future of the number of servers that report themselves as available.
/// This should never error, but would indicate the url that caused the error.
// TODO This would be a good case for using the ! type if the never type stabilizes before this project is due.
pub fn get_num_servers_up() -> impl Future<Item = NumServers, Error = Error> {
    let client = Client::new();

    let uri_1: Uri = "http://129.21.208.2:3000/availability/1".parse().unwrap();
    let uri_2: Uri = "http://129.21.208.2:3000/availability/2".parse().unwrap();
    let uri_3: Uri = "http://129.21.208.2:3000/availability/3".parse().unwrap();
    let uri_4: Uri = "http://129.21.208.2:3000/availability/4".parse().unwrap();

    let request_is_up = |uri: &Uri| {
        client
            .get(uri.clone())
            .and_then(|res| res.into_body().concat2())
            .map(|chunk| {
                let v = chunk.to_vec();
                let body = String::from_utf8_lossy(&v).to_string();
                match body.as_str() {
                    "1" => true, // If the response is just "1" then the service is online.
                    _ => false,
                }
            })
            .or_else(|_err| Ok(false)) // If the endpoint can't be reached, assume that the server isn't available.
    };

    let a_1 = request_is_up(&uri_1).map_err(move |_: ()| Error::DependentConnectionFailed {
        url: uri_1.to_string(),
    });
    let a_2 = request_is_up(&uri_2).map_err(move |_: ()| Error::DependentConnectionFailed {
        url: uri_2.to_string(),
    });
    let a_3 = request_is_up(&uri_3).map_err(move |_: ()| Error::DependentConnectionFailed {
        url: uri_3.to_string(),
    });
    let a_4 = request_is_up(&uri_4).map_err(move |_: ()| Error::DependentConnectionFailed {
        url: uri_4.to_string(),
    });

    a_1.join4(a_2, a_3, a_4).map(|(a, b, c, d)| -> NumServers {
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
        NumServers(acc)
    })
}

/// Gets the "load" on the "servers".
///
/// # Return
/// A Future representing the Load of the "server cluster".
/// If the request fails, it will return an error indicating that that resource is unavailable.
pub fn get_load() -> impl Future<Item = Load, Error = Error> {
    let uri: Uri = "http://129.21.208.2:3000/serverLoad".parse().unwrap();
    let client = Client::new();
    client
        .get(uri.clone())
        .and_then(|res| {
            res.into_body().concat2() // Await the whole body
        })
        .map_err(move |_| Error::DependentConnectionFailed {
            url: uri.to_string(),
        })
        .and_then(|chunk: Chunk| {
            let v = chunk.to_vec();
            let body = String::from_utf8_lossy(&v).to_string();
            body.parse::<u32>()
                .map_err(|_| crate::error::Error::internal_server_error("Could not parse u32 from load".to_string()))
        })
        .map(Load)
}

/// Per the assignment:
/// Each server can handle "10" load units
/// If the current server load exceeds what can be "served", then activate the dimmer (don't display the advertisement)
///
/// # Arguments
/// load_units - The current load across the "servers"
/// available_servers - The number (out of 4) of servers that are available.
///
/// # Return
/// True indicates that the server can serve an add.
/// False indicates that the server cannot serve the add.
pub fn should_serve_adds(load_units: Load, available_servers: NumServers) -> bool {
    const UNITS_PER_SERVER: u32 = 10;
    let available_capacity = available_servers.0 * UNITS_PER_SERVER;
    available_capacity > load_units.0
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
        let _ = get_load().wait();
    }

    #[test]
    fn should_serve() {
        let load = Load(27);
        let available_servers = NumServers(3);
        assert!(should_serve_adds(load, available_servers))
    }
    #[test]
    fn should_not_serve() {
        let load = Load(30);
        let available_servers = NumServers(3);
        assert!(!should_serve_adds(load, available_servers))
    }
}
