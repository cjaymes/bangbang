use chrono::{DateTime, Utc};
use rocket::Data;
use rocket::Request;
use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;

pub struct Logger {
}

#[rocket::async_trait]
impl Fairing for Logger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request
        }
    }
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let now: DateTime<Utc> = Utc::now();
        println!("{}: {}", now, request.to_string());
    }
}
