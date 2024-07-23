-- Add migration script here
CREATE TABLE call_requests(
    id UUID NOT NULL,
    PRIMARY KEY(id),
    user_name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
