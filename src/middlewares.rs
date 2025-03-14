use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response
};
use std::fs::File;
use std::io::Write;

pub async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    let mut file = match File::create("./log.txt") {
        Ok(file) => file,
        Err(why) => panic!("Could not open log.txt: {}", why),
    };

    
    match file.write_all(req.uri().path().as_bytes()) {
        Ok(file) => file,
        Err(why) => panic!("Could not write to log.txt: {}", why),
    }
    println!("receive visitor {:?}", req.uri());

    next.run(req).await
}