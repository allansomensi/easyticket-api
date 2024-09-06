CREATE TABLE tickets (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    requester VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);