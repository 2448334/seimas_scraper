use diesel::{prelude::*, upsert::on_constraint};
use diesel_derive_enum::DbEnum;
use log::{error, debug};

use crate::{schema::{vote_data}, database::connect::establish_connection};

#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = vote_data)]
pub struct VoteData {
    pub id: i32,
    pub person_id: i32,
    pub vote: Option<VoteType>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::PqVoteType"]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

table! {
    missing_vote_ids (vid) {
        vid -> Int4,
    }
}

impl VoteData {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<VoteData>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(vote_data::table)
            .values(self)
            .on_conflict(on_constraint("vote_data_pkey"))
            .do_update()
            .set(self)
            .get_result::<VoteData>(conn).optional();

        match result {
            Ok(session) => { Ok(session) }
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

    pub fn get_missing_vote_ids() -> Result<Vec<i32>, diesel::result::Error> {
        let conn = &mut establish_connection();
        missing_vote_ids::table.select(missing_vote_ids::vid).load::<i32>(conn)
    }

    pub fn open_save(&self) -> Result<Option<VoteData>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }
}

