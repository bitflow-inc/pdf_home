CREATE TABLE IF NOT EXISTS "groups" (
                                       id TEXT PRIMARY KEY NOT NULL,
                                       name TEXT NOT NULL,
                                       order_no INTEGER NOT NULL,
                                       tree TEXT
);

CREATE TABLE IF NOT EXISTS "contents" (
                                         group_id TEXT NOT NULL,
                                         id TEXT NOT NULL,
                                         type TEXT NOT NULL,
                                         content TEXT NOT NULL,
                                         author TEXT NOT NULL,
                                         reg_dt INTEGER NOT NULL,
                                         released INTEGER NOT NULL DEFAULT 0,
                                         primary key (group_id, id)
);

