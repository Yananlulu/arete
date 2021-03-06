use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::{
    errors::JsonResult,
    i18n::locale::{Dao as LocaleDao, Item as Locale},
    orm::Database,
};
use super::super::super::super::request::Administrator;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub lang: String,
    #[validate(length(min = "1"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

#[get("/admin/locales")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<Locale>> {
    let db = db.deref();
    let it = LocaleDao::all(db)?;
    Ok(Json(it))
}

#[get("/admin/locales/<id>")]
pub fn show(_user: Administrator, id: i64, db: Database) -> JsonResult<Locale> {
    let db = db.deref();
    let it = LocaleDao::by_id(db, &id)?;
    Ok(Json(it))
}

#[post("/admin/locales", format = "json", data = "<form>")]
pub fn create(_user: Administrator, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LocaleDao::create(db, &form.lang, &form.code, &form.message)?;
    Ok(Json(()))
}

#[post("/admin/locales/<id>", format = "json", data = "<form>")]
pub fn update(_user: Administrator, id: i64, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LocaleDao::update(db, &id, &form.lang, &form.code, &form.message)?;
    Ok(Json(()))
}

#[delete("/admin/locales/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    LocaleDao::delete(db, &id)?;
    Ok(Json(()))
}
