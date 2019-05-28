extern crate actix;

use std::time::Instant;
use actix::prelude::*;
use actix_web::*;
use actix_web::actix::Message;

use mongo_driver::client::ClientPool;
use std::sync::Arc;
use bson::{
    oid::ObjectId,
    doc,
    bson,
    Document
};

pub struct MongoExecutor(pub Arc<ClientPool>);

pub struct GetDoc {}

impl Message for GetDoc {
    type Result = Result<Document, Error>;
}

impl Actor for MongoExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GetDoc> for MongoExecutor {
    type Result = Result<Document, Error>;

    fn handle(&mut self, _msg: GetDoc, _ctx: &mut Self::Context) -> Self::Result {
        let conn = &self.0.pop();

        let coll = conn.get_collection("test", "test");
        let find_doc = doc!{"_id" => ObjectId::with_string("5c7e6d54c8c449d59d47ea9f").unwrap()};

        let start = Instant::now();

        let mut result = coll.find(&find_doc, None)
            .map_err(|_| error::ErrorInternalServerError("Error while searching document"))
            .unwrap();

        match result.next() {
            None => {
                Err(error::ErrorNotFound("Not Found"))
            },
            Some(Ok(doc)) => {
                println!("Request time: {:?}", start.elapsed());
                Ok(doc)
            },
            Some(Err(_)) => {
                Err(error::ErrorInternalServerError("Internal server error"))
            }
        }
    }
}
