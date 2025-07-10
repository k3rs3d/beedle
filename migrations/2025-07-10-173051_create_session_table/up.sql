CREATE TABLE session (
    session_id UUID PRIMARY KEY,
    user_id INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,

    ip_address TEXT,
    user_agent TEXT,

    cart_data JSONB
);
CREATE INDEX idx_session_expires ON session(expires_at);
