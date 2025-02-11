use std::cell::LazyCell;

use inline_colorization::*;
use time::format_description::{self, BorrowedFormatItem};

#[allow(dead_code)]
pub enum LogType {
    Info,
    Warning,
    Error,
    InfoBG,
    Normal,
}

pub const FORMAT: LazyCell<Vec<BorrowedFormatItem>> =
    LazyCell::new(|| format_description::parse("[year].[month].[day]. [hour]:[minute]:[second]").unwrap());

impl LogType {
    pub fn to_colors(self) -> (&'static str, &'static str) {
        match self {
            Self::Info => (color_green, color_cyan),
            Self::Warning => (color_yellow, color_bright_yellow),
            Self::Error => (color_red, color_bright_red),
            Self::InfoBG => (color_bright_black, color_bright_black),
            Self::Normal => (color_white, color_white),
        }
    }
}

#[macro_export]
macro_rules! color_println {
    ($color:expr, $value_color:expr, $str_in:expr $(, $rest:expr)*) => {
        let mut modified_str: String = $str_in.to_owned();

        $(modified_str = modified_str.replace("{}", &format!("{}{}{}", $value_color, $rest, $color));)*

        println!("{}{}{}", $color, modified_str, color_white);
    };
}

#[macro_export]
macro_rules! log {
    ($color:expr, $sender:expr, $str_in:expr $(, $rest:expr)*) => {
        //println!("{color_cyan}{}{color_green}\tEnviorment: ✅ Getting '{color_cyan}{key}{color_green}' enviorment is successful! ✅{color_white}", Utc::now().format("[%H:%M:%S]"));
        let (color1, color2) = $color.to_colors();
        let mut txt = String::new();
        txt.push_str(color2);
        txt.push_str("[");
        txt.push_str(time::OffsetDateTime::now_utc().format(&crate::logger::FORMAT).unwrap().as_str());
        txt.push_str("]");
        txt.push_str(color1);
        txt.push_str("\t");
        txt.push_str(crate::logger::adjust_string_length($sender, 13).as_str());
        txt.push_str("\t");

        #[allow(unused_mut)]
        let mut modified_str: String = $str_in.to_owned();
        $(modified_str = modified_str.replacen("{}", &format!("{}{}{}", color2, $rest, color1), 1);)*
        txt.push_str(modified_str.as_str());

        println!("{}{}", txt, "\x1B[37m");
    };
}

pub fn adjust_string_length(s: &str, desired_len: usize) -> String {
    let mut s = s.to_owned();
    if s.len() > desired_len {
        s.truncate(desired_len);
    } else {
        s.extend(std::iter::repeat(' ').take(desired_len - s.len()));
    }
    s
}
