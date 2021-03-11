/*
CREATE TABLE IF NOT EXISTS "group" (
	id TEXT PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
	order_no INTEGER NOT NULL,
	tree TEXT
);
CREATE TABLE IF NOT EXISTS "content" (
	group_id TEXT NOT NULL,
	id TEXT NOT NULL,
	type TEXT NOT NULL,
	content TEXT NOT NULL,
	author TEXT NOT NULL,
	reg_dt INTEGER NOT NULL,
	released INTEGER NOT NULL DEFAULT 0,
	primary key (group_id, id)
);

*/

table! {
    group {
        id -> Integer,
        name -> Text,
        order_no -> Integer,
        tree -> Nullable<Text>,
    },
    content {
        group_id -> Text,
        id -> Text,
        content_type -> Text,
        content -> Text,
        reg_author -> Text,
        reg_dt -> Integer,
        upd_author -> Nullable<Text>,
        upd_dt -> Nullable<Integer>,
    },

}