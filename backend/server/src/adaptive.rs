//! API routes related to serving ads and recording health data about them.
//! (Self adaptive instructions section)[http://www.se.rit.edu/~swen-344/projects/selfadaptive/selfadaptive.html]
//!
//! This module is much more understandable with the aid of this (helpful future reference)[https://rufflewind.com/img/rust-futures-cheatsheet.html].
use crate::error::Error;
use hyper::{
    rt::{Future, Stream},
    Chunk, Client, Uri,
};

use futures::future::join_all;
use apply::Apply;
use futures::future::Either;
use serde::{Serialize,Deserialize};
use crate::state::HttpsClient;

/// The fictional load encountered by the servers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Load(pub u32);
/// The fictional number of servers currently available.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumServers(pub u32);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    availability: u32
}



/// Checks to see if a given server is available.
///
/// It resolves any error in making the connection or deserializing the value to a false.
fn server_is_available(uri: Uri, client: &HttpsClient) -> Box<Future<Item=bool, Error=()> + 'static + Send> {
    server_is_available_core(uri, client)
        .map(|resp| {
            match resp {
                Ok(resp) => match resp.availability {
                    1 => true,  // One indicates that the server is up
                    _ => false,
                }
                Err(_) => false // Request deserialization failed
            }
        })
        .or_else(|_err| Ok(false)) // If the endpoint can't be reached, assume that the server isn't available.
        .apply(Box::new) // This needs to be boxed in order for multiple different futures to be joined
}

/// Checks if the server is available.
fn server_is_available_core(uri: Uri, client: &HttpsClient) -> impl Future<Item=Result<AvailabilityResponse, serde_json::Error>, Error=hyper::Error> {
        client
            .get(uri.clone())
            .and_then(|res| res.into_body().concat2()) // Get the whole body.
            .map(|chunk| {
                let v: Vec<u8> = chunk.to_vec();
                serde_json::from_slice::<AvailabilityResponse>(&v)
            })
            .apply(Box::new)
}



/// Get the number of available servers.
///
/// # Return
/// A Future of the number of servers that report themselves as available.
/// This should never error, but would indicate the url that caused the error.
pub fn get_num_servers_up(client: &HttpsClient) -> impl Future<Item = NumServers, Error = Error> {

    let uris = vec![
        "https://adaptive-server.herokuapp.com/availability/1",
        "https://adaptive-server.herokuapp.com/availability/2",
        "https://adaptive-server.herokuapp.com/availability/3",
        "https://adaptive-server.herokuapp.com/availability/4",
    ]
        .into_iter()
        .map(|s| s.parse::<Uri>().map_err(|e| Error::internal_server_error(format!("Malformed uri in get_num_servers_up:  {:?}", e))))
        .collect::<Result<Vec<_>, Error>>();

    let uris = match uris {
        Ok(uris) => uris,
        Err(e) => return Either::A(futures::future::err(e)) // Return early with the parse error.
    };

    let servers: Vec<Box<Future<Item=bool, Error=()> + 'static + Send>> = uris
        .into_iter()
        .map(|uri|server_is_available(uri, &client))
        .collect();

    join_all(servers) // Wait for all requests to finish.
        .map(|x: Vec<bool>| {
            // Sum the number of servers that responded with a positive message.
            x
                .into_iter()
                .fold(0, |acc, b| -> u32 {
                    acc + b as u32
                })
                .apply(NumServers)
        })
        .map_err(|_| Error::InternalServerError(None)) // This can never error, but Type Coherency must be maintained
        .apply(Either::B) // Use Either here to remove the need for boxing.
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
            body.parse::<u32>().map_err(|_| {
                crate::error::Error::internal_server_error(
                    "Could not parse u32 from load".to_string(),
                )
            })
        })
        .map(Load)
}

/// Per the assignment:
/// Each server can handle "10" load units
/// If the current server load exceeds what can be "served", then activate the dimmer (don't display the advertisement)
///
/// # Arguments
/// * load_units - The current load across the "servers"
/// * available_servers - The number (out of 4) of servers that are available.
///
/// # Return
/// * True indicates that the server can serve an add.
/// * False indicates that the server cannot serve the add.
#[cfg(test)]
pub fn should_serve_adds(load_units: Load, available_servers: NumServers) -> bool {
    const UNITS_PER_SERVER: u32 = 10;
    let available_capacity = available_servers.0 * UNITS_PER_SERVER;
    available_capacity > load_units.0
}

/// Per the assignment:
/// Each server can handle "10" load units
/// If the current server load exceeds what can be "served", then activate the dimmer (don't display the advertisement)
///
/// # Arguments
/// * load_units - The current load across the "servers"
/// * available_servers - The number (out of 4) of servers that are available.
///
/// # Return
/// * True indicates that the server can serve an add.
/// * False indicates that the server cannot serve the add.
///
/// # Note
/// Because of the obtuse nature of the implementation, this function is a likely source of bugs.
pub fn should_serve_adds_bf(load_units: Load, available_servers: NumServers) -> bool {
    let bf_code = r###"
    +                   // Increment cell 0 by 1, to make the comparison below change from an effective => to an >.
    > // Shift to cell 1
    [>++++++++++< -]    // Multiply cell 1 by 10, store in cell 2
    <                   // Move ptr to cell 0
    [>+< -]             // Shift contents of cell 0 left to cell 1
    >                   // Move ptr to cell 1

    // At this point, the load+1 is stored in cell 1, and the capacity is stored in cell 2

    [->-[>]<<]         // Repeatedly ubtract 1 from cells 0,1 until either is 0.
    // This will leave a value in cell 2 if the capacity > load. TODO check that
    // If cell 2 is non-zero, the add should be served
    "###;

    let bf_program = bf::parse_brainfuck(bf_code).unwrap(); // This is assumed to be safe.
    let mut tape = vec![0; 10];
    tape[0] = load_units.0 as u8;
    tape[1] = available_servers.0 as u8;
    bf::run_brainfuck(&bf_program, &mut tape);

    // Cell 2 represents a boolean
    tape[2] != 0
}


#[cfg(test)]
mod test {
    use super::*;
    use hyper_tls::HttpsConnector;
//    use futures::future::lazy;

    #[test]
    fn check_if_server_is_available() {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, _>(https);
        tokio::runtime::current_thread::Runtime::new().unwrap().block_on(
            server_is_available_core("https://adaptive-server.herokuapp.com/availability/1".parse().unwrap(), &client)
                .and_then(|resp| {
                    dbg!(&resp);
                    let availability = resp.expect("could not parse").availability;
                    assert!(availability == 1 || availability == 0);
                    Ok(())
                })
                .map_err(|_|())
        )
            .expect("future should finish")
    }

    #[test]
    fn num_servers_up() {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, _>(https);
        assert!(get_num_servers_up(&client).wait().is_ok())
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
    fn should_serve_bf_equivalent() {
        let load = Load(27);
        let available_servers = NumServers(3);
        assert_eq!(should_serve_adds(load, available_servers), should_serve_adds_bf(load, available_servers))
    }



    #[test]
    fn should_not_serve() {
        let load = Load(30);
        let available_servers = NumServers(3);
        assert!(!should_serve_adds(load, available_servers))
    }

    #[test]
    fn should_not_serve_bf_equivalent() {
        let load = Load(30);
        let available_servers = NumServers(3);
        assert_eq!(should_serve_adds(load, available_servers), should_serve_adds_bf(load, available_servers))
    }
}
