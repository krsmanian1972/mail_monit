mod event;
mod ferris_mail;
mod sendgrid_mail;
mod service;

use event::{create_event_file, EVENT_DIR};
use service::{get_pending_mails, send_mails};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    std::env::set_var("RUST_LOG", "mail_monit=info");
    env_logger::init();
    dotenv::dotenv().ok();
    std::fs::create_dir_all(EVENT_DIR).unwrap();

    dispatch_pending_mails().await;

    Ok(())
}

async fn dispatch_pending_mails() {
    let response = get_pending_mails().await;

    if response.is_ok() {
        send_mails(&response.unwrap()).await;
    }
}
