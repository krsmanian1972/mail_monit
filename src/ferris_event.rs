#![allow(non_snake_case)]

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

use chrono::format::strftime::StrftimeItems;
use chrono::{NaiveDateTime, Utc};

use base64::encode;
use ics::properties::{Description, DtEnd, DtStart, Method, Organizer, Attendee, Sequence, Status, Summary, URL};
use ics::{Event, ICalendar};

pub const EVENT_DIR: &str = "/Users/pmpower/assets/events";

#[derive(Serialize, Deserialize, Debug)]
pub struct FerrisEvent {
    pub id: String,
    pub description: String,
    
    #[serde(default)]
    pub organizer: String,

    #[serde(default)]
    pub attendee: String,
    
    #[serde(default)]
    pub sequence: i32,

    pub startDate: String,
    pub endDate: String,
    pub status: String,
    pub method: String,
}

impl FerrisEvent {
    /**
     * The FerrisEvent when transformed into attachment, please provide
     * the subject of the correspondence to fill the Summary of the .ics attachment file
     */
    pub fn as_attachment(&self, subject: &str) -> std::io::Result<HashMap<String, String>> {
        let cal_content = self.calendar_content(subject)?;
        let file_name = format!("{}.ics", &self.id);

        let mut attachment: HashMap<String, String> = HashMap::new();
        attachment.insert(String::from("filename"), file_name);
        attachment.insert(String::from("content"), encode(cal_content));
        attachment.insert(String::from("type"), String::from("text/calendar"));
        attachment.insert(String::from("disposition"), String::from("attachment"));

        Ok(attachment)
    }
    fn as_ical_event(&self, subject: &str) -> ICalendar {
        let now = Utc::now().naive_utc();
        let sequence: String = self.sequence.to_string();
        let mut event = Event::new(&self.id, format_time(now));
        event.push(Summary::new(subject.to_owned()));
        event.push(Description::new(&self.description));
        event.push(Organizer::new(&self.organizer));
        event.push(Attendee::new(&self.attendee));
        event.push(Sequence::new(sequence));
        event.push(DtStart::new(&self.startDate));
        event.push(DtEnd::new(&self.endDate));
        event.push(Method::new(&self.method));
        event.push(Status::new(&self.status));

        event.push(URL::new("https://krscode.com"));

        let mut calendar = ICalendar::new("2.0", "ics-rs");
        calendar.add_event(event);

        calendar
    }
    fn calendar_content(&self, subject: &str) -> std::io::Result<String> {
        let calendar = self.as_ical_event(subject);

        let path = format!("{}/{}.ics", EVENT_DIR, &self.id);
        calendar.save_file(&path)?;

        let mut buffer = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}

pub fn format_time(given_time: NaiveDateTime) -> String {
    let fmt = StrftimeItems::new("%Y%m%dT%H%M%SZ");
    given_time.format_with_items(fmt.clone()).to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create_attachment_map() {

        use crate::ferris_mail::Correspondence;

        let corres = Correspondence {
            id: "12-12-12".to_owned(),
            fromEmail: "krs@krscode.com".to_owned(),
            subject: "subject-1".to_owned(),
            content: r#"{"id":"r1-r2-r3","description":"test_desc","organizer":"raja","startDate":"20201020T161500Z","endDate":"20201020T163000Z","status":"CONFIRMED","method":"CONFIRMED"}"#
                .to_owned(),
            mailType: "event".to_owned(),
        };

        let ferris_event = corres.as_event().unwrap();

        let attachment: HashMap<String,String> = ferris_event.as_attachment(&corres.subject).unwrap();

        assert_eq!("r1-r2-r3.ics",attachment.get("filename").unwrap());
    }
}
