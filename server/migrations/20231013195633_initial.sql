-- Add migration script here
CREATE TABLE IF NOT EXISTS letters (
    id UUID PRIMARY KEY,
    message TEXT NOT NULL,
    to_user_id UUID NOT NULL,
    by_user_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    sending_info_id UUID
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS sending_infos (
    id UUID PRIMARY KEY,
    letter_id UUID NOT NULL UNIQUE,

    sent_at TIMESTAMPTZ,
    received_at TIMESTAMPTZ,

    from_loc TEXT NOT NULL,
    to_loc TEXT NOT NULL,
    eta TIMESTAMPTZ NOT NULL
);