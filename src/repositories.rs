use crate::models::{NewRustac, Rustac};
use crate::schema::rustacs;
use diesel::prelude::*;

pub struct RustacRepo;

impl RustacRepo {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustac> {
        rustacs::table.find(id).get_result::<Rustac>(c)
    }

    pub fn find_multiple(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rustac>> {
        rustacs::table
            .order(rustacs::id.desc())
            .limit(limit)
            .load::<Rustac>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_rustac: NewRustac) -> QueryResult<Rustac> {
        diesel::insert_into(rustacs::table)
            .values(new_rustac)
            .execute(c)?;

        let last_id = Self::last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    pub fn save(c: &mut SqliteConnection, id: i32, rustac: Rustac) -> QueryResult<Rustac> {
        diesel::update(rustacs::table.find(id))
            .set((
                rustacs::name.eq(rustac.name.to_owned()),
                rustacs::email.eq(rustac.email.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustacs::table.find(id)).execute(c)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        rustacs::table
            .select(rustacs::id)
            .order(rustacs::id.desc())
            .first(c)
    }
}
