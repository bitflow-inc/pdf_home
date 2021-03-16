table! {
    contents (group_id, id) {
        group_id -> Text,
        id -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        content -> Text,
        author -> Text,
        reg_dt -> Integer,
        released -> Integer,
    }
}

table! {
    groups (id) {
        id -> Text,
        name -> Text,
        order_no -> Text,
        tree -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    contents,
    groups,
);
