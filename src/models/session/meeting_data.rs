use diesel::prelude::*;
use chrono::NaiveDateTime;
use log::{error, debug};

use crate::{schema::{meeting_data, agenda_item, vote, speech, registration}, database::connect::establish_connection};

#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = meeting_data)]
pub struct MeetingData {
    pub id: i32,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,

    pub agenda: Vec<Option<i32>>,
    pub registrations: Vec<Option<i32>>,
}

#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = agenda_item)]
pub struct AgendaItem {
    pub id: i32,
    pub agenda_state_id: Option<i32>,
    pub agenda_group_id: Option<i32>,
    pub document_key: Option<i32>,
    pub nr: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    pub agenda_type: Option<String>,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,

    pub speeches: Vec<Option<i32>>,
    pub voting: Vec<Option<i32>>,
}

#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = vote)]
pub struct Vote {
    pub id: i32,
    pub summary: Option<String>,
    pub result: Option<String>,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = speech)]
pub struct Speech {
    pub id: i32,
    pub discussion_id: Option<i32>,
    pub person_id: Option<i32>,
    pub person: Option<String>,
    pub office: Option<String>,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Queryable, AsChangeset)]
#[diesel(table_name = registration)]
pub struct Registration {
    pub id: i32,
    pub result: Option<String>,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
}


impl MeetingData {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<MeetingData>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(meeting_data::table)
            .values(self)
            .on_conflict(meeting_data::id)
            .do_update()
            .set(self)
            .get_result::<MeetingData>(conn).optional();

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

    pub fn open_save(&self) -> Result<Option<MeetingData>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }
}


impl AgendaItem {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<AgendaItem>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(agenda_item::table)
            .values(self)
            .on_conflict(agenda_item::id)
            .do_update()
            .set(self)
            .get_result::<AgendaItem>(conn).optional();

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

    pub fn open_save(&self) -> Result<Option<AgendaItem>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }
}


impl Vote {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<Vote>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(vote::table)
            .values(self)
            .on_conflict(vote::id)
            .do_update()
            .set(self)
            .get_result::<Vote>(conn).optional();

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

    pub fn open_save(&self) -> Result<Option<Vote>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }


    pub fn get_vote_ids() -> Result<Vec<i32>, diesel::result::Error> {
        let conn = &mut establish_connection();
        vote::table.select(vote::id).load::<i32>(conn)
    }
}


impl Speech {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<Speech>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(speech::table)
            .values(self)
            .on_conflict(speech::id)
            .do_update()
            .set(self)
            .get_result::<Speech>(conn).optional();

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

    pub fn open_save(&self) -> Result<Option<Speech>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }
}


impl Registration {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<Registration>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(registration::table)
            .values(self)
            .on_conflict(registration::id)
            .do_update()
            .set(self)
            .get_result::<Registration>(conn).optional();

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

    pub fn open_save(&self) -> Result<Option<Registration>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }

    pub fn get_registration_ids() -> Result<Vec<i32>, diesel::result::Error> {
        let conn = &mut establish_connection();
        registration::table.select(registration::id).load::<i32>(conn)
    }
}
