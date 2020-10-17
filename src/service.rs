use std::env;
use actix_web::client::Client;
use serde::{Deserialize, Serialize};


use crate::ferris_mail::{FerrisMail, FerrisResponse, GET_FERRIS_MAILS_QUERY};
use crate::sendgrid_mail::SendGridMail;

const FERRIS_URL: &'static str = "http://localhost:8088/graphql";

pub async fn send_mails(ferris_response: &FerrisResponse) {
    
    let mails = ferris_response.data.getSendableMails.mails.as_ref().unwrap();
    
    for mail in mails {
        let sendgrid_mail = to_sendgrid_mail(&mail);
        let sendgrid_response = dispatch_mail(&sendgrid_mail).await;
    }
}

pub fn to_sendgrid_mail(given_mail: &FerrisMail) -> SendGridMail {
    let corres = &given_mail.correspondence;
    let people = &given_mail.receipients;

    let mail_from = corres.fromEmail.as_str();
    let mail_subject = corres.subject.as_str();
    let mail_content = corres.content.as_str();

    let mut to_emails: Vec<String> = Vec::new();
    let mut cc_emails: Vec<String> = Vec::new();
    let mut bcc_emails: Vec<String> = Vec::new();

    for person in people {
        let to_type = person.toType.as_str();
        let email = person.toEmail.as_str();

        if to_type.eq_ignore_ascii_case("to") {
            to_emails.push(email.to_owned());
        } else if to_type.eq_ignore_ascii_case("cc") {
            cc_emails.push(email.to_owned());
        } else if to_type.eq_ignore_ascii_case("bcc") {
            bcc_emails.push(email.to_owned());
        } else {
            cc_emails.push(email.to_owned());
        }
    }

    let mail = SendGridMail::new(
        mail_from,
        &to_emails,
        &cc_emails,
        &bcc_emails,
        mail_subject,
        mail_content,
    );

    mail
}

#[derive(Serialize, Deserialize)]
pub struct GraphQL {
    query: String,
}

pub async fn get_pending_mails() -> Result<FerrisResponse, String> {
    let graph_ql = GraphQL {
        query: GET_FERRIS_MAILS_QUERY.to_owned(),
    };

    let body_data = serde_json::to_string(&graph_ql).unwrap();

    let ferris_client = Client::default();

    let response_result = ferris_client
        .post(FERRIS_URL)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send_body(&body_data)
        .await;

    if response_result.is_err() {
        return Err("Error while requesting for pending mails".to_owned());
    }

    let body_result = response_result.unwrap().body().await;

    if body_result.is_err() {
        return Err("Error while unpacking the response for body".to_owned());
    }

    let body_bytes = body_result.unwrap();
    let s = std::str::from_utf8(&body_bytes).expect("utf8 parse error)");

    let ferris_response: FerrisResponse = serde_json::from_str(&s).unwrap();

    Ok(ferris_response)
}

pub async fn dispatch_mail(mail: &SendGridMail) -> Result<String,String>{

    let mail_data = serde_json::to_string(&mail).unwrap();

    let api_key = env::var("SENDGRID_API_KEY").expect("The Sendgrid API Key should be set");
    let sendgrid_url = env::var("SENDGRID_URL").expect("The Sendgrid URL is not set");
    
    let auth = format!("{} {}", "Bearer", api_key);
    let sendgrid_client = Client::default();

    let response = sendgrid_client
        .post(sendgrid_url)
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .send_body(&mail_data)
        .await;

    
    if response.is_ok() {
        return Ok(String::from("Ok"));
    }

    Err(String::from("Error in Mail Dispatch"))
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_rt;

    #[actix_rt::test]
    pub async fn test_should_send_mail() {
        println!("Calling Test Mail Dispatch Method");

        let from = "test@krscode.com";

        let to_emails = vec![String::from("krsmanian1972@gmail.com")];

        let cc_emails: Vec<String> = Vec::new();
        let bcc_emails: Vec<String> = Vec::new();

        let subject = "Unit Testing the API with Html Content";
        let content = test_html_body();

        let mail = SendGridMail::new(from, &to_emails, &cc_emails, &bcc_emails, subject, content.as_str());

        let result = dispatch_mail(&mail).await;

        assert_eq!("Ok", result.unwrap());
    }

    fn test_html_body() -> String {
        let content = r#"
        <html>
            <body>
                <h5>Welcome to Ferris - The Coaching Assistant</h5>
                <br/>
                <p>Hundreds of companies around the world are using Rust in production today for fast, low-resource, cross-platform solutions. Software you know and love, like&nbsp;<a href="https://hacks.mozilla.org/2017/08/inside-a-super-fast-css-engine-quantum-css-aka-stylo/" rel="noopener noreferrer" target="_blank">Firefox</a>,&nbsp;<a href="https://blogs.dropbox.com/tech/2016/06/lossless-compression-with-brotli/" rel="noopener noreferrer" target="_blank">Dropbox</a>, and&nbsp;<a href="https://blog.cloudflare.com/cloudflare-workers-as-a-serverless-rust-platform/" rel="noopener noreferrer" target="_blank">Cloudflare</a>, uses Rust.&nbsp;<strong>From startups to large corporations, from embedded devices to scalable web services, Rust is a great fit.</strong></p><p><br></p><p>My biggest compliment to Rust is that it's boring, and this is an amazing compliment.</p><p class="ql-align-right">– Chris Dickinson, Engineer at npm, Inc</p><p class="ql-align-center"><a href="https://www.npmjs.com/" rel="noopener noreferrer" target="_blank"><img src="https://www.rust-lang.org/static/images/user-logos/npm.svg"></a></p><p class="ql-align-center"><a href="https://www.youtube.com/watch?v=u6ZbF4apABk" rel="noopener noreferrer" target="_blank"><img src="https://www.rust-lang.org/static/images/user-logos/yelp.png"></a></p><p>All the documentation, the tooling, the community is great - you have all the tools to succeed in writing Rust code.</p><p class="ql-align-right">– Antonio Verardi, Infrastructure Engineer</p><p><br></p>
            </body>
        </html>
        "#;

        String::from(content)
    }
}
