use diesel::{prelude::*, upsert::on_constraint};
use log::{debug, error};

use crate::{schema::registration_data, database::connect::establish_connection};

#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = registration_data)]
pub struct RegistrationData {
    pub id: i32,
    pub person_id: i32,
    pub registered: Option<bool>,
}

table! {
    missing_registration_ids (rid) {
        rid -> Int4,
    }
}

impl RegistrationData {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<RegistrationData>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(registration_data::table)
            .values(self)
            .on_conflict(on_constraint("registration_data_pkey"))
            .do_update()
            .set(self)
            .get_result::<RegistrationData>(conn).optional();

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

    
    pub fn get_missing_registration_ids() -> Result<Vec<i32>, diesel::result::Error> {
        let conn = &mut establish_connection();
        missing_registration_ids::table.select(missing_registration_ids::rid).load::<i32>(conn)
    }


    pub fn open_save(&self) -> Result<Option<RegistrationData>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }
}

