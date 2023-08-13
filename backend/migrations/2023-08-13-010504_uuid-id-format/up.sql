-- Your SQL goes here
CREATE FUNCTION gen_random_app_id(column_id varchar(5)) RETURNS TEXT AS $$
BEGIN
  RETURN column_id || '|' || gen_random_uuid();
END;
$$ LANGUAGE plpgsql;

ALTER TABLE users
    ALTER COLUMN id SET DEFAULT gen_random_app_id('user');

ALTER TABLE refresh_tokens
    ALTER COLUMN id SET DEFAULT gen_random_app_id('rtokn');
