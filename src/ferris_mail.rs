#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Correspondence {
  pub id: String,
  pub fromEmail: String,
  pub subject: String,
  pub content: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct MailRecipient {
  pub toEmail: String,
  pub toType: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct FerrisMail {
  pub correspondence: Correspondence,
  pub receipients: Vec<MailRecipient>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct GetSendableMails {
  pub mails: Option<Vec<FerrisMail>>,
  pub error: Option<String>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Data {
  pub getSendableMails: GetSendableMails
}

#[derive(Serialize, Deserialize,Debug)]
pub struct FerrisResponse {
  pub data: Data
}

pub const GET_FERRIS_MAILS_QUERY: &'static str = r#"query  {
    getSendableMails {
      mails {
        correspondence
        {
          id
          fromEmail
          subject
          content
        }
        receipients
        {
          toType
          toEmail
        }
      }
      error {
        message
      }
    }
  }"#;
