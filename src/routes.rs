#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}
