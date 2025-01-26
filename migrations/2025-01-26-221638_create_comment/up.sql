CREATE TYPE comment_type AS ENUM ('submission', 'assignment', 'module', 'course');

-- Table
CREATE TABLE comments
(
    id         TEXT PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta       JSONB                    DEFAULT '{}',

    user_id    TEXT REFERENCES accounts (id),
    owner_id   TEXT,
    content    TEXT,
    type       comment_type
);

-- Indexes for `comments`
CREATE INDEX idx_comments_user_id ON comments (user_id);
CREATE INDEX idx_comments_user_id_type ON comments (user_id, type);
CREATE INDEX idx_comments_owner_id ON comments (owner_id);
CREATE INDEX idx_comments_owner_id_type ON comments (owner_id, type);