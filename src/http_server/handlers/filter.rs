use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use ical::generator::Emitter;
use thiserror::Error;
use tokio::io::{self};

use std::{
    io::Cursor,
    ops::{Deref, Not},
};

/// A hibák kezeléséhez egy enum
#[derive(Error, Debug)]
pub enum GetError {
    #[error("Invalid Id")]
    InvalidId,
    #[error("Other error")]
    OtherError,
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] ical::parser::ParserError),
    #[error("IO Error: {0}")]
    IOError(#[from] io::Error),
    #[error("{0}: {1}")]
    StatusError(StatusCode, String),
}
impl IntoResponse for GetError {
    fn into_response(self) -> Response {
        match self {
            _ => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
        }
    }
}

pub async fn request_handler(Path(id): Path<String>) -> Result<Response<String>, GetError> {
    filter(id, false).await
}
pub async fn inverse_request_handler(Path(id): Path<String>) -> Result<Response<String>, GetError> {
    filter(id, true).await
}

async fn filter(id: String, inverse: bool) -> Result<Response<String>, GetError> {
    let id = id.replace(".ics", "");

    if id.len() < 10 {
        return Err(GetError::InvalidId);
    }

    let url = format!(
        "https://{}/hallgato/api/Calendar/CalendarExportFileToSyncronization?id={id}.ics",
        crate::DOMAIN.deref()
    );
    let response = reqwest::get(url).await?;

    let status = response.status();

    if status.is_success().not() {
        let txt = response.text().await?;
        return Err(GetError::StatusError(status, txt));
    }

    let og_header = response.headers().clone();
    let array = response.bytes().await?;
    let reader = Cursor::new(array);

    let mut parser = ical::IcalParser::new(reader);

    let mut calendar = match parser.next() {
        Some(a) => a?,
        None => {
            return Err(GetError::OtherError);
        }
    };
    if let Some(summary) = calendar.properties.iter_mut().find(|x| x.name == "X-WR-CALNAME") {
        if inverse {
            summary.value = Some("Neptun (Inverse Filtered) - by Feri".to_string());
        } else {
            summary.value = Some("Neptun (filtered) - by Feri".to_string());
        }
    };

    let mut filtered_events = Vec::new();
    for event in calendar.events {
        let Some(summary) = event.properties.iter().find(|x| x.name == "SUMMARY") else {
            continue;
        };
        let Some(summary) = &summary.value else {
            continue;
        };
        if !summary.ends_with("False") && !inverse {
            filtered_events.push(event);
        } else if summary.ends_with("False") && inverse {
            filtered_events.push(event);
        }
    }
    calendar.events = filtered_events;

    let mut re = Response::new(calendar.generate());
    let header = re.headers_mut();
    let any = [
        "expires",
        "content-disposition",
        "access-control-expose-headers",
        "strict-transport-security",
        "content-type",
        "pragma",
        "cache-control",
    ];
    for (name, value) in og_header {
        let Some(name) = name else {
            continue;
        };

        if any.contains(&name.as_str()) {
            header.append(name, value);
        }
    }

    Ok(re)
}
