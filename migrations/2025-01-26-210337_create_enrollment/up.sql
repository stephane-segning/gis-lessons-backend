CREATE TYPE enrollment_type AS ENUM ('student', 'teacher');

-- Table
CREATE TABLE enrollments
(
    id              TEXT PRIMARY KEY,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta            JSONB                    DEFAULT '{}',

    user_id         TEXT            NOT NULL REFERENCES accounts (id),
    course_id       TEXT            NOT NULL REFERENCES courses (id),
    enrollment_type enrollment_type NOT NULL
);

-- Indexes for `enrollments`
CREATE INDEX idx_enrollments_user_id ON enrollments (user_id);
CREATE INDEX idx_enrollments_user_id_enrollment_type ON enrollments (user_id, enrollment_type);
CREATE INDEX idx_enrollments_course_id ON enrollments (course_id);
CREATE INDEX idx_enrollments_course_id_enrollment_type ON enrollments (course_id, enrollment_type);