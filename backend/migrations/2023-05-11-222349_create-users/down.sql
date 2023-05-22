-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS users_username_index;

DROP TABLE IF EXISTS users;