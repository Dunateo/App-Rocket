#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/daam", rank=2)]
fn daam() -> &'static str {
    "On est dans un autre controller"
}



#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello, daam])
}
