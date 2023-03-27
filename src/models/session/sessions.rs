use diesel::prelude::*;
use chrono::{NaiveDate};
use log::{debug, error};

use crate::{schema::sessions, database::connect::establish_connection};


#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = sessions)]
pub struct Sessions {
    pub id: i32,
    pub num: i32,
    pub name: String,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub parliament: i32,
}


impl Sessions {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<Sessions>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(sessions::table)
            .values(self)
            .on_conflict(sessions::id)
            .do_update()
            .set(self)
            .get_result::<Sessions>(conn).optional();

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

    pub fn open_save(&self) -> Result<Option<Sessions>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }

    pub fn get_sessions_per_parliament(parliament_id: i32) -> Result<Vec<i32>, diesel::result::Error> {
        let conn = &mut establish_connection();
        sessions::table.filter(sessions::parliament.eq(parliament_id)).select(sessions::id).load::<i32>(conn)
    }
}

