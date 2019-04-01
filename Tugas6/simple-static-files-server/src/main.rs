extern crate config;
extern crate rouille;

use rouille::Response;
use rouille::Server;

use std::net::{SocketAddr};

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();

    let port = settings.get_int("http_port").unwrap();
    let serve_directory = settings.get_str("dir").unwrap();
    println!("Listening to localhost:{:?}. \nDirectory served {:?}", port, serve_directory);

    let cert = include_bytes!("../server.crt").to_vec();
    let pkey = include_bytes!("../server.key").to_vec();

    let addr = SocketAddr::from(([127, 0, 0, 1], settings.get_int("http_port").unwrap() as u16));

    Server::new_ssl(addr, move |request| {
        {
            let response = rouille::match_assets(&request, ".");
            if response.is_success() {
                return response;
            }
        }

        Response::html("File Not Found!!!.")
            .with_status_code(404)
    }
    
    , cert, pkey).unwrap().run();

}
