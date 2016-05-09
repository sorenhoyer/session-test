 #![feature(type_ascription)]
 
extern crate iron;
extern crate session;

use std::net::{SocketAddr, Ipv4Addr};
use session::{Sessions, SessionStore, HashSessionStore, Session};
use session::sessions::RequestSession;

use iron::{Request, Response, IronResult, Chain, Iron};
//use iron::prelude::*;
use std::boxed::Box;
use std::any::Any;
use iron::status::{Status};
use iron::typemap;
use std::marker::PhantomData;

fn handle_request(req: &mut Request) -> IronResult<Response> {
    // Retrieve our session from the store
    let session = req.extensions.get_mut::<RequestSession<MySessionKey>>();

    let mut count = 0;
    match session {
        None => {
            println!("{}", "session is 'None'")
        },
        Some(v) => {
            println!("{:?}", 2);
            
            match v.find() {
                None => {
                    println!("{}", "v.find() none")
                },
                Some(v2) => {
                    // Store or increase the sessioned count
                    count = v.upsert(v2, count_func)
                }
            }
            
        },
    }

    println!("{} hits from\t{}", count, req.remote_addr);

    Ok(Response::with((iron::status::Ok, format!("Sessioned count: {:?}", count))))
}

fn count_func(v: &mut u32) {  
    *v = *v + 1 
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct MySessionKey;
impl typemap::Key for MySessionKey { type Value = u32; }

fn id_generator(_: &Request) -> u32 { 1 }

fn main() {
    let mut chain = Chain::new(handle_request);

    let hs: HashSessionStore<MySessionKey> = HashSessionStore::new();
    let s: Sessions<MySessionKey, HashSessionStore<MySessionKey>> = Sessions::new(id_generator, hs);
    chain.link_before(s);
    let mut server = Iron::new(chain);
    server.http("localhost:3000");
}