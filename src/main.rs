#[macro_use] extern crate rocket;
use std::net::SocketAddr;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_ip_address])
}

#[get("/")]
fn get_ip_address(remote_address: SocketAddr) -> String {
    format!("ip: {:?}", remote_address.ip())
}
