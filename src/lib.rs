#[macro_use]
extern crate nom;

use std::{fmt, str};

mod helpers;
mod mail;
mod parse_helpers;
mod parser;

use helpers::bytes_to_dbg;
pub use mail::MailCommand;
pub use parser::command as parse_command; // TODO: give a nicer interface

// TODO: escape initial '.' in DataItem by adding another '.' in front (and opposite when
// receiving)

#[cfg_attr(test, derive(PartialEq))]
pub struct DataCommand<'a> {
    // Still SMTP-escaped (ie. leading ‘.’ doubled) message
    data: &'a [u8],
}

#[cfg_attr(test, derive(PartialEq))]
pub struct EhloCommand<'a> {
    domain: &'a [u8],
}

#[cfg_attr(test, derive(PartialEq))]
pub struct HeloCommand<'a> {
    domain: &'a [u8],
}

#[cfg_attr(test, derive(PartialEq))]
pub struct RcptCommand<'a> {
    // TO: parameter with the “@ONE,@TWO:” portion removed, as per RFC5321 Appendix C
    to: &'a [u8],
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Command<'a> {
    Data(DataCommand<'a>), // DATA <CRLF>
    Ehlo(EhloCommand<'a>), // EHLO <domain> <CRLF>
    Helo(HeloCommand<'a>), // HELO <domain> <CRLF>
    Mail(MailCommand<'a>), // MAIL FROM:<@ONE,@TWO:JOE@THREE> [SP <mail-parameters>] <CRLF>
    Rcpt(RcptCommand<'a>), // RCPT TO:<@ONE,@TWO:JOE@THREE> [SP <rcpt-parameters] <CRLF>
}

impl<'a> fmt::Debug for DataCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "DataCommand {{ data: {} }}", bytes_to_dbg(self.data))
    }
}

impl<'a> fmt::Debug for EhloCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "EhloCommand {{ domain: {} }}", bytes_to_dbg(self.domain))
    }
}

impl<'a> fmt::Debug for HeloCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "HeloCommand {{ domain: {} }}", bytes_to_dbg(self.domain))
    }
}

impl<'a> fmt::Debug for RcptCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "RcptCommand {{ to: {} }}", bytes_to_dbg(self.to))
    }
}
