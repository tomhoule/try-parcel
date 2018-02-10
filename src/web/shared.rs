use askama::Template;
use diesel::prelude::*;
use diesel::r2d2::*;
use rocket::State;
use r2d2;

pub type DbPool<'a> = State<'a, r2d2::Pool<ConnectionManager<PgConnection>>>;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Base;
