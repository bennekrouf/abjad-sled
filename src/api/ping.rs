use rocket::get;

#[get("/ping")]
pub fn ping() -> &'static str {
    "Service accessibility test : OK!"
}