-- Your SQL goes here
CREATE TABLE refresh_tokens (
   id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
   user_id UUID NOT NULL REFERENCES users(id),
   expires_at TIMESTAMP NOT NULL DEFAULT NOW() + INTERVAL '1 week',
   created_at TIMESTAMP NOT NULL DEFAULT NOW(),
   updated_at TIMESTAMP,
   UNIQUE (user_id)
)