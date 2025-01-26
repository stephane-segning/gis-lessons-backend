-- Table
CREATE TABLE lessons
(
    id         TEXT PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta       JSONB                    DEFAULT '{}',

    module_id  TEXT  NOT NULL REFERENCES modules (id),
    title      TEXT  NOT NULL,
    content    JSONB NOT NULL
);

-- Indexes for `lessons`
CREATE INDEX idx_lessons_title ON lessons (title);
CREATE INDEX idx_lessons_module_id ON lessons (module_id);