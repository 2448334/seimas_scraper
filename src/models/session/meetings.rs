use diesel::prelude::*;
use chrono::NaiveDateTime;
use log::{debug, error};

use crate::{schema::meetings, database::connect::establish_connection};


#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = meetings)]
pub struct Meetings {
    pub id: i32,
    pub num: i32,
    pub meeting_type: String,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
    pub session: i32,
    pub protocol_link: Option<String>,
    pub stenogram_link: Option<String>,
    pub video_comment: Option<String>,
    pub video_link: Option<String>,
}

table! {
    missing_meeting_ids (mid) {
        mid -> Int4,
    }
}

impl Meetings {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<Meetings>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(meetings::table)
            .values(self)
            .on_conflict(meetings::id)
            .do_update()
            .set(self)
            .get_result::<Meetings>(conn).optional();

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

    pub fn open_save(&self) -> Result<Option<Meetings>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }    

    pub fn get_meetings_ids(session_id: i32) -> Result<Vec<i32>, diesel::result::Error> {
        let conn = &mut establish_connection();
        meetings::table.filter(meetings::session.eq(session_id)).select(meetings::id).load::<i32>(conn)
    }

    pub fn get_session_count(session_id: i32) -> Result<i64, diesel::result::Error> {
        let conn = &mut establish_connection();
        meetings::table.filter(meetings::session.eq(session_id)).count().get_result::<i64>(conn)
    }

    pub fn get_protocols_per_session(session_id: i32) -> Result<Vec<(i32, i32, Option<String>)>, diesel::result::Error> {
        let conn = &mut establish_connection();
        meetings::table.filter(meetings::session.eq(session_id)).select((meetings::session, meetings::num, meetings::protocol_link)).load::<(i32, i32, Option<String>)>(conn)
    }

    pub fn get_stenograms_per_session(session_id: i32) -> Result<Vec<(i32, i32, Option<String>)>, diesel::result::Error> {
        let conn = &mut establish_connection();
        meetings::table.filter(meetings::session.eq(session_id)).select((meetings::session, meetings::num, meetings::stenogram_link)).load::<(i32, i32, Option<String>)>(conn)
    }

    pub fn get_missing_meeting_ids() -> Result<Vec<i32>, diesel::result::Error> {
        let conn = &mut establish_connection();
        missing_meeting_ids::table.select(missing_meeting_ids::mid).load::<i32>(conn)
    }
}
