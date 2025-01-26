-- Table
CREATE TABLE assignments
(
    id          TEXT PRIMARY KEY,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta        JSONB                    DEFAULT '{}',

    lesson_id   TEXT                     NOT NULL REFERENCES lessons (id),
    title       TEXT                     NOT NULL,
    description TEXT,
    due_date    TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Indexes for `assignments`
CREATE INDEX idx_assignments_title ON assignments (title);
CREATE INDEX idx_assignments_lesson_id ON assignments (lesson_id);