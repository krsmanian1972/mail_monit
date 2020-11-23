use serde::{Deserialize, Serialize};
use std::collections::HashMap;
     
#[derive(Serialize, Deserialize)]
pub struct Personalizations {
    pub to: Option<Vec<HashMap<String, String>>>,
    pub cc: Option<Vec<HashMap<String, String>>>,
    pub bcc: Option<Vec<HashMap<String, String>>>,
}

impl Personalizations {
    pub fn personal_map(emails: &[String]) -> Option<Vec<HashMap<String, String>>> {
        if emails.is_empty(){
            return None;
        }

        let mut result = Vec::new();
        for email in emails {
            let mut map = HashMap::new();
            map.insert(String::from("email"), email.clone());

            result.push(map);
        }

        Some(result)
    }

    pub fn new(to_emails: &[String], cc_emails: &[String], bcc_emails: &[String]) -> [Personalizations; 1] {
        let to = Personalizations::personal_map(to_emails);
        let cc = Personalizations::personal_map(cc_emails);
        let bcc = Personalizations::personal_map(bcc_emails);

        let personalization = Personalizations { to, cc, bcc };

        [personalization]
    }
}

#[derive(Serialize, Deserialize)]
pub struct SendGridMail {
    pub from: HashMap<String, String>,
    pub personalizations: [Personalizations; 1],
    pub subject: String,
    pub content: [HashMap<String, String>; 1],
    pub attachments: Option<Vec<HashMap<String,String>>>
}

impl SendGridMail {

    pub fn new(from: &str, to_emails: &[String], cc_emails: &[String], bcc_emails: &[String], subject: &str, body: &str) -> SendGridMail {
        let mut from_map = HashMap::new();
        from_map.insert(String::from("email"), String::from(from));

        let personalizations = Personalizations::new(to_emails, cc_emails, bcc_emails);

        let mut body_map = HashMap::new();
        body_map.insert(String::from("type"), String::from("text/html"));
        body_map.insert(String::from("value"), String::from(body));

        let content = [body_map; 1];

        SendGridMail {
            from: from_map,
            personalizations,
            subject: String::from(subject),
            content,
            attachments: None,
        }
    }
}

