use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use serde_json::{from_value, to_value, Value};

use super::super::super::super::{
    errors::Result,
    orm::{
        schema::{survey_fields, survey_forms, survey_logs, survey_responses},
        Connection,
    },
};

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub type_: Value,
    pub nbf: NaiveDate,
    pub exp: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn type_(self) -> Result<Type> {
        let it = from_value(self.type_)?;
        Ok(it)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Type {
    pub public: bool,
    pub multiple: bool,
}

pub trait Dao {
    fn add(
        &self,
        user: &i64,
        title: &String,
        description: &String,
        nbf: &NaiveDate,
        exp: &NaiveDate,
        type_: &Type,
    ) -> Result<i64>;

    fn update(
        &self,
        id: &i64,
        title: &String,
        description: &String,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<()>;
    fn get(&self, id: &i64) -> Result<Item>;
    fn delete(&self, id: &i64) -> Result<()>;
    fn latest(&self) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(
        &self,
        user: &i64,
        title: &String,
        description: &String,
        nbf: &NaiveDate,
        exp: &NaiveDate,
        type_: &Type,
    ) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let id = insert_into(survey_forms::dsl::survey_forms)
            .values((
                survey_forms::dsl::user_id.eq(user),
                survey_forms::dsl::title.eq(title),
                survey_forms::dsl::description.eq(description),
                survey_forms::dsl::nbf.eq(nbf),
                survey_forms::dsl::exp.eq(exp),
                survey_forms::dsl::type_.eq(&to_value(type_)?),
                survey_forms::dsl::updated_at.eq(&now),
            ))
            .returning(survey_forms::dsl::id)
            .get_result(self)?;
        Ok(id)
    }

    fn update(
        &self,
        id: &i64,
        title: &String,
        description: &String,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = survey_forms::dsl::survey_forms.filter(survey_forms::dsl::id.eq(id));
        update(it)
            .set((
                survey_forms::dsl::title.eq(title),
                survey_forms::dsl::description.eq(description),
                survey_forms::dsl::nbf.eq(nbf),
                survey_forms::dsl::exp.eq(exp),
                survey_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn get(&self, id: &i64) -> Result<Item> {
        let it = survey_forms::dsl::survey_forms
            .filter(survey_forms::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }

    fn delete(&self, id: &i64) -> Result<()> {
        delete(survey_fields::dsl::survey_fields.filter(survey_fields::dsl::form_id.eq(id)))
            .execute(self)?;
        delete(
            survey_responses::dsl::survey_responses.filter(survey_responses::dsl::form_id.eq(id)),
        )
        .execute(self)?;
        delete(survey_logs::dsl::survey_logs.filter(survey_logs::dsl::form_id.eq(id)))
            .execute(self)?;
        delete(survey_forms::dsl::survey_forms.filter(survey_forms::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }

    fn latest(&self) -> Result<Vec<Item>> {
        let items = survey_forms::dsl::survey_forms
            .order(survey_forms::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
