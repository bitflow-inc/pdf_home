extern crate serde_derive;  // 1.0.117
extern crate serde;         // 1.0.117
extern crate serde_json;    // 1.0.59
extern crate actix_web;
extern crate proc_macro;

use actix_web::{body::Body, web, App, HttpRequest, HttpResponse, HttpServer, Result, Responder, Error, http::Method, web::Json};
use std::{env, borrow::Cow, sync::mpsc, thread, path::Path, fs, io::Write, str, ffi::OsString, ffi::OsStr};
use serde::{Deserialize, Serialize};
use super::super::db as db;
use super::super::db::model:: { Groups, Contents, };
use diesel::prelude::*;


pub async fn get_group(req: HttpRequest) -> impl Responder {

    println!("get_group");

    use super::super::db::schema::groups::dsl::*;

    let connection = db::establish_connection();
    let results = groups
        // .filter(published.eq(true))
        .limit(5)
        .load::<Groups>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} groups", results.len());

    let mut ret:Vec<Groups> = Vec::new();
    for item in results {
        println!("{} / {} / {}", item.id, item.name, item.order_no);
        ret.push(item);
    }

    // let ret = Vec::new();
    return web::Json(ret);

}

pub async fn set_group(req: HttpRequest) -> Result<HttpResponse> {

    println!("set_group");
    Ok(HttpResponse::Ok().json({}))
}

pub async fn put_group(req: HttpRequest) -> Result<HttpResponse> {

    println!("put_group");
    Ok(HttpResponse::Ok().json({}))
}

pub async fn del_group(req: HttpRequest) -> Result<HttpResponse> {

    println!("del_group");
    Ok(HttpResponse::Ok().json({}))
}
