use chrono::NaiveDate;
use diesel::prelude::*;
use log::{error, trace};
use crate::{schema::parliament, database::connect::establish_connection};

#[derive(Insertable, Debug, Queryable)]
#[diesel(table_name = parliament)]
pub struct Parliament {
    pub id: i32,
    pub name: Option<String>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}


impl Parliament {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<Parliament>, diesel::result::Error> {
        trace!("Saving {:?}", self);
        let result = diesel::insert_into(parliament::table)
            .values(self)
            .on_conflict(parliament::id)
            .do_nothing()
            .get_result::<Parliament>(conn).optional();

        match result {
            Ok(data) => { Ok(data) }
            Err(error) => {
                match &error {
                    diesel::result::Error::DatabaseError(error_type, error_info) => {
                        match (error_type, error_info) {
                            (diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                                Ok(None)
                            }
                            (_, _) => {
                                error!("{:?}\n{:?}", self, error);
                                return Err(error);
                            }
                        }
                    }
                    _ => {
                        error!("{:?}\n{:?}", self, error);
                        return Err(error);
                    }
                }
            }
        }
    }

    pub fn open_save(&self) -> Result<Option<Parliament>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }

    pub fn get_parliaments_ids() -> Result<Vec<i32>, diesel::result::Error> {
        let conn = &mut establish_connection();
        parliament::table.select(parliament::id).load::<i32>(conn)
    }
}
