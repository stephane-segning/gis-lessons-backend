CREATE TABLE courses
(
    id          TEXT PRIMARY KEY,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta        JSONB                    DEFAULT '{}',

    name        TEXT NOT NULL,
    slug        TEXT NOT NULL UNIQUE,
    description TEXT NULL
);

-- Indexes for `courses`
CREATE INDEX idx_courses_name ON courses (name);
CREATE INDEX idx_courses_slug ON courses (slug);
