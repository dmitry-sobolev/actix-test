extern crate actix;
extern crate actix_web;
extern crate env_logger;

use std::sync::Arc;

use actix::prelude::*;
use actix_web::{App, http, HttpResponse, middleware, server};
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::State;
use futures::Future;
use mongo_driver::client::{ClientPool, Uri};

use mongo::GetDoc;
use mongo::MongoExecutor;
use settings::Settings;

mod mongo;

mod settings;

struct AppState {
    db: Addr<MongoExecutor>,
}

fn get(
    (_req, state): (HttpRequest<AppState>, State<AppState>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    state.db
        .send(GetDoc {})
        .from_err()
        .and_then(|res| match res {
            Ok(doc) => Ok(HttpResponse::Ok().json(doc)),
            Err(_e) => {
//                println!("{:?}", e);
                Ok(HttpResponse::InternalServerError().body("123").into())
            }
        })
}


fn main() {
    ::std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    println!("{:?}", Settings::load());

    let sys = actix::System::new("mongo-test");

    let uri = Uri::new("mongodb://user:pass@localhost:27017/?compressors=zlib&maxpoolsize=10").unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));

    let addr = SyncArbiter::start(1, move || MongoExecutor(pool.clone()));

    let srv = server::new(move ||
        App::with_state(AppState { db: addr.clone() })
            .middleware(middleware::Logger::default())
            .resource("/", |r| {
                r.method(http::Method::GET).with_async(get)
            }))
        .bind("127.0.0.1:8080").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();

    sys
}