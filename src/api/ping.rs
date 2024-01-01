use rocket::get;

#[get("/ping")]
pub fn ping() -> &'static str {
    "Simple content route reached"
}