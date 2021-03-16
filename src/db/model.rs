use diesel::prelude::*;
use chrono::naive::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Groups {
    pub id: String,
    pub name: String,
    pub order_no: String,
    pub tree: Option<String>,
}

#[derive(Serialize, Queryable)]
pub struct Contents {
    pub group_id: String,
    pub id: String,
    pub content_type: String,
    pub content: String,
    pub reg_author: String,
    pub reg_dt: NaiveDateTime,
    pub upd_author: Option<String>,
    pub upd_dt: Option<NaiveDateTime>,
}
