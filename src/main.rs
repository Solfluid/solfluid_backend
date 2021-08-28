use rocket::{launch, routes};
mod payment_stream;
mod routes;
mod solana;
use routes::get_streams;
use routes::index;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_streams])
}
