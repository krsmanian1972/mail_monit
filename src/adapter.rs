
use std::collections::HashMap;

use crate::ferris_event::FerrisEvent;
use crate::ferris_mail::FerrisMail;
use crate::sendgrid_mail::SendGridMail;

/**
 * Transform the given FerrisMail structure into SendGridMail Structure
 *
 * The transformed sendgrid mail structure will have
 * a placeholder for attachment.
 */
pub fn to_sendgrid_mail(ferris_mail: &FerrisMail) -> SendGridMail {
    let corres = &ferris_mail.correspondence;
    let people = &ferris_mail.receipients;

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

    SendGridMail::new(mail_from, &to_emails, &cc_emails, &bcc_emails, mail_subject, mail_content)
}

/**
 * 1. Convert the given ferris mail into sendgrid mail
 *
 * 2. Then push the ferris event as attachment into the sendgrid mail.
 *
 * Note:
 * The content of the FerrisMail is the JSON structure to create the FerrisEvent.
 *
 * The FerrisEvent we receive from Ferris does not have the subject (summary), hence
 * we will supplement it from the ferris_mail
 *
 */
pub fn to_sendgrid_event_mail(ferris_mail: &mut FerrisMail) -> SendGridMail {
    let ferris_event: FerrisEvent = ferris_mail.correspondence.as_event().unwrap();

    // The content of the mail is the description of event
    ferris_mail.correspondence.content = ferris_event.description.to_owned();
    let mut sendgrid_mail = to_sendgrid_mail(ferris_mail);

    // The summary of the event is the subject of the mail
    let subject = ferris_mail.correspondence.subject.as_str();
    let attachment = ferris_event.as_attachment(subject).unwrap();

    let mut attachments: Vec<HashMap<String, String>> = Vec::new();
    attachments.push(attachment);
    sendgrid_mail.attachments = Some(attachments);

    sendgrid_mail
}


