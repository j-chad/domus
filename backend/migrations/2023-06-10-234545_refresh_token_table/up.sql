-- Your SQL goes here
ALTER TABLE users ALTER COLUMN id type UUID USING GEN_RANDOM_UUID();
ALTER TABLE users ALTER COLUMN id set default GEN_RANDOM_UUID();

CREATE TABLE refresh_tokens (
   id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
   user_id uuid NOT NULL REFERENCES users(id),
   expires_at TIMESTAMP NOT NULL DEFAULT NOW() + INTERVAL '1 week',
   created_at TIMESTAMP NOT NULL DEFAULT NOW(),
   updated_at TIMESTAMP,
   UNIQUE (user_id)
)