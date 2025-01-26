-- Table
CREATE TABLE modules
(
    id          TEXT PRIMARY KEY,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta        JSONB                    DEFAULT '{}',

    course_id   TEXT NOT NULL REFERENCES courses (id),
    title       TEXT NOT NULL,
    description TEXT NULL
);

-- Indexes for `modules`
CREATE INDEX idx_modules_title ON modules (title);
CREATE INDEX idx_modules_course_id ON modules (course_id);
