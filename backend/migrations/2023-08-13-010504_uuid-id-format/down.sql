-- This file should undo anything in `up.sql`
DROP FUNCTION gen_random_app_id(column_id varchar(5));

ALTER TABLE users
    ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE refresh_tokens
    ALTER COLUMN id SET DEFAULT gen_random_uuid();
