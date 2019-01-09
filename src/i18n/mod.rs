pub mod locale;

use std::ops::Deref;
use std::time::Duration;

use mustache;
use rocket::{
    request::{self, FromRequest},
    Outcome, Request,
};
use serde::ser::Serialize;

use super::{
    cache::Cache,
    errors::{Error, Result},
    orm::{Database, PooledConnection as DbConnection},
    redis::{PooledConnection as RedisConnection, Redis},
};

use self::locale::Dao;

pub struct I18n {
    pub db: DbConnection,
    pub cache: RedisConnection,
}

impl I18n {
    pub fn languages(&self) -> Result<Vec<String>> {
        self.cache.get(
            &"languages".to_string(),
            Duration::from_secs(60 * 60 * 24 * 7),
            || -> Result<Vec<String>> { self.db.deref().languages() },
        )
    }

    pub fn exist(&self, lang: &String) -> bool {
        if let Ok(items) = self.languages() {
            return items.contains(lang);
        }
        false
    }

    fn get(&self, lang: &String, code: &String) -> Result<Option<String>> {
        self.cache.get(
            &format!("locales.{}.{}", lang, code),
            Duration::from_secs(60 * 60 * 24 * 7),
            || -> Result<Option<String>> {
                if let Ok(it) = self.db.deref().by_lang_and_code(lang, code) {
                    return Ok(Some(it.message));
                }
                Ok(None)
            },
        )
    }

    pub fn tr<S: Serialize>(
        &self,
        lang: &String,
        code: &String,
        args: &Option<S>,
    ) -> Result<Option<String>> {
        if let Some(msg) = self.get(lang, code)? {
            let tpl = mustache::compile_str(&msg)?;
            return Ok(Some(tpl.render_to_string(args)?));
        }
        Ok(None)
    }

    pub fn e<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> Error {
        self.t(lang, code, args).into()
    }

    pub fn t<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> String {
        if let Some(msg) = match self.tr(lang, code, args) {
            Ok(v) => v,
            Err(e) => {
                error!("{:?}", e);
                None
            }
        } {
            return msg;
        }
        format!("{}.{}", lang, code)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for I18n {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let Database(db) = request.guard::<Database>()?;
        let Redis(cache) = request.guard::<Redis>()?;

        Outcome::Success(I18n {
            db: db,
            cache: cache,
        })
    }
}
