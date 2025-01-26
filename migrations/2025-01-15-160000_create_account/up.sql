-- Table
CREATE TABLE accounts
(
    id         TEXT PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta       JSONB                    DEFAULT '{}',

    sub        TEXT NOT NULL,
    name       TEXT NOT NULL
);

-- Indexes for `accounts`
CREATE INDEX idx_accounts_sub ON accounts (sub);
CREATE INDEX idx_accounts_name ON accounts (name);
