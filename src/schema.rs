// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pq_department_type"))]
    pub struct PqDepartmentType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pq_gender"))]
    pub struct PqGender;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pq_vote_type"))]
    pub struct PqVoteType;
}

diesel::table! {
    agenda_item (id) {
        id -> Int4,
        agenda_state_id -> Nullable<Int4>,
        agenda_group_id -> Nullable<Int4>,
        document_key -> Nullable<Int4>,
        nr -> Nullable<Text>,
        name -> Nullable<Text>,
        state -> Nullable<Text>,
        agenda_type -> Nullable<Text>,
        from -> Nullable<Timestamp>,
        to -> Nullable<Timestamp>,
        speeches -> Array<Nullable<Int4>>,
        voting -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    meeting_data (id) {
        id -> Int4,
        from -> Nullable<Timestamp>,
        to -> Nullable<Timestamp>,
        agenda -> Array<Nullable<Int4>>,
        registrations -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    meetings (id) {
        id -> Int4,
        num -> Int4,
        meeting_type -> Text,
        from -> Nullable<Timestamp>,
        to -> Nullable<Timestamp>,
        session -> Int4,
        protocol_link -> Nullable<Text>,
        stenogram_link -> Nullable<Text>,
        video_comment -> Nullable<Text>,
        video_link -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PqDepartmentType;

    office (id) {
        id -> Int4,
        department_id -> Nullable<Int4>,
        department_name -> Nullable<Text>,
        department_type -> Nullable<PqDepartmentType>,
        duties -> Nullable<Text>,
        from -> Nullable<Date>,
        to -> Nullable<Date>,
    }
}

diesel::table! {
    parliament (id) {
        id -> Int4,
        name -> Nullable<Text>,
        from -> Nullable<Date>,
        to -> Nullable<Date>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PqGender;

    politician (id, parliament) {
        id -> Int4,
        parliament -> Int4,
        name -> Text,
        surname -> Text,
        gender -> Nullable<PqGender>,
        from -> Nullable<Date>,
        to -> Nullable<Date>,
        party -> Nullable<Text>,
        elected_type -> Nullable<Text>,
        biography_link -> Nullable<Text>,
        term_count -> Nullable<Int4>,
        email -> Nullable<Text>,
        phone -> Array<Nullable<Text>>,
        website -> Nullable<Text>,
        offices -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    registration (id) {
        id -> Int4,
        result -> Nullable<Text>,
        from -> Nullable<Timestamp>,
        to -> Nullable<Timestamp>,
    }
}

diesel::table! {
    registration_data (id, person_id) {
        id -> Int4,
        person_id -> Int4,
        registered -> Nullable<Bool>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        num -> Int4,
        name -> Text,
        from -> Nullable<Date>,
        to -> Nullable<Date>,
        parliament -> Int4,
    }
}

diesel::table! {
    speech (id) {
        id -> Int4,
        discussion_id -> Nullable<Int4>,
        person_id -> Nullable<Int4>,
        person -> Nullable<Text>,
        office -> Nullable<Text>,
        from -> Nullable<Timestamp>,
        to -> Nullable<Timestamp>,
    }
}

diesel::table! {
    vote (id) {
        id -> Int4,
        summary -> Nullable<Text>,
        result -> Nullable<Text>,
        from -> Nullable<Timestamp>,
        to -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PqVoteType;

    vote_data (id, person_id) {
        id -> Int4,
        person_id -> Int4,
        vote -> Nullable<PqVoteType>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    agenda_item,
    meeting_data,
    meetings,
    office,
    parliament,
    politician,
    registration,
    registration_data,
    sessions,
    speech,
    vote,
    vote_data,
);
