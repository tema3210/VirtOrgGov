table! {
    use diesel::pg::types::sql_types::Uuid;
    use diesel::sql_types::*;

    codexes (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    use diesel::pg::types::sql_types::Uuid;
    use diesel::sql_types::*;

    law (id) {
        author -> Uuid,
        codex -> Uuid,
        id -> Uuid,
    }
}

table! {
    use diesel::pg::types::sql_types::Uuid;
    use diesel::sql_types::*;

    law_data (submit_date, what) {
        what -> Uuid,
        new_text -> Varchar,
        submit_date -> Timestamp,
        status -> Varchar,
    }
}

table! {
    use diesel::pg::types::sql_types::Uuid;
    use diesel::sql_types::*;

    parties (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    use diesel::pg::types::sql_types::Uuid;
    use diesel::sql_types::*;

    politics (id) {
        id -> Uuid,
        name -> Varchar,
        surname -> Varchar,
        lastname -> Varchar,
        age -> Int2,
        party -> Uuid,
    }
}

joinable!(law -> codexes (codex));
joinable!(law -> politics (author));
joinable!(law_data -> law (what));
joinable!(politics -> parties (party));

allow_tables_to_appear_in_same_query!(
    codexes,
    law,
    law_data,
    parties,
    politics,
);
