use actix::prelude::*;

mod adapter;
mod ferris_event;
mod ferris_mail;
mod sendgrid_mail;
mod service;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use service::{get_pending_mails, send_mails};
use std::time::{Duration, SystemTime};

#[warn(unused_variables)]
async fn index(_request: HttpRequest) -> HttpResponse {
    let body = "Welcome to Mail Monitor.";
    HttpResponse::Ok().body(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let bind = dotenv::var("BIND").unwrap();
    let polling_seconds = dotenv::var("POLLING_SECONDS").unwrap().parse::<u64>().unwrap();
    let duration = Duration::from_secs(polling_seconds);

    std::fs::create_dir_all(ferris_event::EVENT_DIR).unwrap();

    let arbiter = Arbiter::new();

    Actor::start_in_arbiter(&arbiter, move |ctx| {
        ctx.run_interval(duration, move |_, c: &mut Context<Executor>| c.address().do_send(Ping { ts: SystemTime::now() }));

        Executor {
            last_message_sent: SystemTime::UNIX_EPOCH,
        }
    });

    HttpServer::new(move || App::new().route("/", web::get().to(index))).bind(&bind)?.run().await
}

struct Ping {
    ts: SystemTime,
}

impl Message for Ping {
    type Result = ();
}

struct Executor {
    last_message_sent: SystemTime,
}

impl Actor for Executor {
    type Context = Context<Self>;
}

impl Handler<Ping> for Executor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        self.last_message_sent = msg.ts;

        Box::pin(dispatch_pending_mails().into_actor(self).map(|res, _act, _ctx| {
            println!("Message {:?}", res);
        }))
    }
}

async fn dispatch_pending_mails() {
    let mail_result = get_pending_mails().await;

    match mail_result {
        Ok(pending_mails) => {
            if pending_mails.is_empty() {
                println!("No Pending Mails");
            } else {
                println!("Number of Mails gathered {}", pending_mails.len());
                send_mails(pending_mails).await;
            }
        }
        Err(e) => {
            eprintln!("Error while gathering mails {}", e);
            std::process::exit(0);
        }
    }
}
