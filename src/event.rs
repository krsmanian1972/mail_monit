use chrono::format::strftime::StrftimeItems;
use chrono::{Duration, NaiveDateTime,Utc};

use ics::properties::{Comment, Description, URL, DtEnd, DtStart, Status, Summary};
use ics::{Event, ICalendar, ToDo};

pub const EVENT_DIR: &'static str = "/Users/pmpower/assets/events";

pub fn create_todo_file(id: &str) -> std::io::Result<()> {
    let mut calendar = ICalendar::new("2.0", "ics-rs");

    let mut todo = ToDo::new(id, "20201018T190000");
    todo.push(Summary::new("Session Title Goes Here"));
    todo.push(Comment::new("Agenda Goes Here"));
    todo.push(Status::needs_action());

    calendar.add_todo(todo);

    let path = format!("{}/{}.ics", EVENT_DIR, id);
    calendar.save_file(path)?;

    Ok(())
}

pub fn format_time(given_time: NaiveDateTime) -> String {
    let fmt = StrftimeItems::new("%Y%m%dT%H%M%SZ");

    given_time.format_with_items(fmt.clone()).to_string()
}

pub fn create_event_file(id: &str) -> std::io::Result<()> {
    let mut calendar = ICalendar::new("2.0", "ics-rs");
    let duration = Duration::minutes(30 as i64);

    let now = Utc::now().naive_utc();
    let start_date = now.checked_add_signed(duration).unwrap();
    let end_date = start_date.checked_add_signed(duration).unwrap();

    let mut event = Event::new("5a33-45a3-b8a9-99d108d64fcb", format_time(now));
    event.push(Summary::new("Session Summary of the ICS Testing"));
    event.push(Description::new("ICS testing using the ics crate. This crate will be used for exchaning events with the people involved."));
    event.push(DtStart::new(format_time(start_date)));
    event.push(DtEnd::new(format_time(end_date)));
    event.push(URL::new("https://krscode.com"));

    calendar.add_event(event);

    let path = format!("{}/{}.ics", EVENT_DIR, id);
    calendar.save_file(path)?;

    Ok(())
}
