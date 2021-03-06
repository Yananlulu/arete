use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{schema::links, Connection},
};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub href: String,
    pub label: String,
    pub loc: String,
    pub lang: String,
    pub x: i16,
    pub y: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: &i64) -> Result<Item>;
    fn create(
        &self,
        lang: &String,
        label: &String,
        href: &String,
        loc: &String,
        x: &i16,
        y: &i16,
    ) -> Result<()>;
    fn update(
        &self,
        id: &i64,
        lang: &String,
        label: &String,
        href: &String,
        loc: &String,
        x: &i16,
        y: &i16,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: &i64) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: &i64) -> Result<Item> {
        let it = links::dsl::links
            .filter(links::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &self,
        lang: &String,
        label: &String,
        href: &String,
        loc: &String,
        x: &i16,
        y: &i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(links::dsl::links)
            .values((
                links::dsl::lang.eq(lang),
                links::dsl::loc.eq(loc),
                links::dsl::href.eq(href),
                links::dsl::label.eq(label),
                links::dsl::x.eq(x),
                links::dsl::y.eq(y),
                links::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: &i64,
        lang: &String,
        label: &String,
        href: &String,
        loc: &String,
        x: &i16,
        y: &i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(links::dsl::links.filter(links::dsl::id.eq(id)))
            .set((
                links::dsl::lang.eq(lang),
                links::dsl::loc.eq(loc),
                links::dsl::href.eq(href),
                links::dsl::label.eq(label),
                links::dsl::x.eq(x),
                links::dsl::y.eq(y),
                links::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = links::dsl::links
            .order(links::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: &i64) -> Result<()> {
        delete(links::dsl::links.filter(links::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
