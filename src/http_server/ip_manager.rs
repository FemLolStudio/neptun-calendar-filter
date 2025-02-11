use std::net::SocketAddr;
//std::net::IpAddr,
//std::str::FromStr,

use axum::http::HeaderMap;
//use lib_core::logger::LogType;
// use md5::{Digest, Md5};

pub fn get_user_ip(addr: &SocketAddr, headers: &HeaderMap) -> String {
    //let headers = request.headers().unwrap();
    //lib_core::log!(
    //    LogType::InfoBG,
    //    "GetUserIp",
    //    "ℹ Source address: {} Headers: {} ℹ",
    //    format!("{}", addr),
    //    format!("{:?}", headers)
    //);
    let add = &addr.ip().to_string();
    let ipaddress = 
    // if let Some(value) = headers.get("cf-connecting-ip") {
    //     value.to_str().unwrap_or(add)
    // } else 
    if let Some(value) = headers.get("x-real-ip") {
        value.to_str().unwrap_or(add)
    } 
    // else if let Some(value) = headers.get("client-ip") {
    //     value.to_str().unwrap_or(add)
    // } else if let Some(value) = headers.get("x-forwarded-for") {
    //     value.to_str().unwrap_or(add)
    // } else if let Some(value) = headers.get("x-forwarded") {
    //     value.to_str().unwrap_or(add)
    // } else if let Some(value) = headers.get("x-cluster-client-ip") {
    //     value.to_str().unwrap_or(add)
    // } else if let Some(value) = headers.get("forwarded-for") {
    //     value.to_str().unwrap_or(add)
    // } else if let Some(value) = headers.get("forwarded") {
    //     value.to_str().unwrap_or(add)
    // } 
    else {
        add
    };
    ipaddress.to_owned()
}
