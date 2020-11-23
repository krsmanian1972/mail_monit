#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::ferris_event::FerrisEvent;

pub const GET_FERRIS_MAILS_QUERY: &str = r#"query  {
  getSendableMails {
    mails {
      correspondence
      {
        id
        fromEmail
        subject
        content
        mailType
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

#[derive(Serialize, Deserialize)]
pub struct GraphQL {
  query: String,
}

impl GraphQL {
  pub fn pending_mails_query() -> GraphQL {
    GraphQL {
      query: GET_FERRIS_MAILS_QUERY.to_owned(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FerrisMail {
  pub correspondence: Correspondence,
  pub receipients: Vec<MailRecipient>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Correspondence {
  pub id: String,
  pub fromEmail: String,
  pub subject: String,
  pub content: String,
  pub mailType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MailRecipient {
  pub toEmail: String,
  pub toType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSendableMails {
  pub mails: Option<Vec<FerrisMail>>,
  pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  pub getSendableMails: GetSendableMails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FerrisResponse {
  pub data: Data,
}

impl Correspondence {
  
  pub fn as_event(&self) -> std::io::Result<FerrisEvent> {

    let ferris_event: FerrisEvent = serde_json::from_str(&self.content)?;
  
    Ok(ferris_event)
  }
}


#[cfg(test)]
mod tests {

  use super::*;

  /**
   * The mails of type event uses the content field to carry a json structure. 
   * We shall transform the json into FerrisEvent.
   */
  #[test]
  fn should_parse_event() {
    let corres = Correspondence{
      id: "12-12-12".to_owned(),
      fromEmail: "krs@krscode.com".to_owned(),
      subject: "subject-1".to_owned(),
      content: r#"{"id":"r1-r2-r3","description":"test_desc","organizer":"raja","startDate":"20201020T161500Z","endDate":"20201020T163000Z","status":"CONFIRMED","method":"CONFIRMED"}"#.to_owned(),
      mailType: "event".to_owned(),
    };

    let ferris_event = corres.as_event().unwrap();
    assert_eq!("r1-r2-r3",ferris_event.id);
    assert_eq!("raja",ferris_event.organizer);
    assert_eq!("CONFIRMED",ferris_event.method);
  }

  /**
   * Example test when we add new fields in the future. The
   * old data will not have the fields. The parsing should be compatible
   */
  #[test]
  fn should_parse_event_with_missing_organizer() {

    let corres_legacy = Correspondence{
      id: "10-10-10".to_owned(),
      fromEmail: "krs@krscode.com".to_owned(),
      subject: "subject-y".to_owned(),
      content: r#"{"id":"r1-r2-r3","description":"test_desc","startDate":"20201020T161500Z","endDate":"20201020T163000Z","status":"CONFIRMED","method":"CONFIRMED"}"#.to_owned(),
      mailType: "event".to_owned(),
    };

    let ferris_event = corres_legacy.as_event().unwrap();
    assert_eq!("",ferris_event.organizer);
  } 
}

