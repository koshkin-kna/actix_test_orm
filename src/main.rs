extern crate actix;
extern crate actix_web;
extern crate actix_web_ult;
extern crate env_logger;
extern crate regex;
#[macro_use]
extern crate tera;

use actix_web::server;

#[cfg(debug_assertions)]
use std::env;

pub mod app;
pub mod urls;
pub mod views;

fn main() {
    match env::args().nth(1) {
        Some(ref x) if x == "makemigrations" => makemigrations(),
        Some(ref x) if x == "migrate" => migrate(),
        Some(ref x) => println!("Wrong flag: {}", x),
        None => run()
    }
}

fn makemigrations() {
    println!("Command - makemigrations - start");
    println!("Command - makemigrations - end");
}

fn migrate() {
    println!("Command - migrate - start");
    println!("Command - migrate - end");
}

fn run() {
    #[cfg(debug_assertions)] {
        env::set_var("RUST_LOG", "actix_web=debug");
        env::set_var("RUST_BACKTRACE", "0");
        env_logger::init();
    }

    let sys = actix::System::new("ultimate");
    let _addr = server::new(|| app::create_app())
        .bind("127.0.0.1:8000")
        .expect("Can not bind to 127.0.0.1:8000")
        .start();
    let _ = sys.run();
}