use diesel::{prelude::*, upsert::on_constraint};
use chrono::NaiveDate;
use log::{debug, error};

use crate::{schema::{politician, office}, database::connect::establish_connection};
use diesel_derive_enum::DbEnum;

#[derive(Insertable, Queryable, Identifiable, Debug, PartialEq, AsChangeset)]
#[diesel(table_name = politician)]
pub struct Politician {
    pub id: i32,
    pub parliament: i32,
    pub name: String,
    pub surname: String,
    pub gender: Option<Gender>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub party: Option<String>,
    pub elected_type: Option<String>,
    pub biography_link: Option<String>,
    pub term_count: Option<i32>,
    pub email: Option<String>,
    pub phone: Vec<Option<String>>,
    pub website: Option<String>,
    pub offices: Vec<Option<i32>>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[diesel(table_name = office)]
pub struct OfficeInsertable {
    pub department_id: Option<i32>,
    pub department_name: Option<String>,
    pub department_type: Option<DepartmentType>,
    pub duties: Option<String>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = office)]
pub struct OfficeQueryable {
    pub id: i32,
    pub department_id: Option<i32>,
    pub department_name: Option<String>,
    pub department_type: Option<DepartmentType>,
    pub duties: Option<String>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::PqDepartmentType"]
pub enum DepartmentType {
    Office,
    Group,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::PqGender"]
pub enum Gender {
    M,
    F,
}


impl Politician {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<Politician>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(politician::table)
            .values(self)
            .on_conflict(on_constraint("politician_pkey"))
            .do_update()
            .set(self)
            .get_result::<Politician>(conn).optional();

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
}


impl OfficeInsertable {
    pub fn save(&self, conn: &mut PgConnection) -> Result<Option<OfficeQueryable>, diesel::result::Error> {
        debug!("Saving {:?}", self);
        let result = diesel::insert_into(office::table)
            .values(self)
            .on_conflict(office::id)
            .do_update()
            .set(self)
            .get_result::<OfficeQueryable>(conn).optional();

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

    pub fn open_save(&self) -> Result<Option<OfficeQueryable>, diesel::result::Error> {
        let conn = &mut establish_connection();
        self.save(conn)
    }
}

