CREATE TYPE submission_member_role AS ENUM ('main', 'member');

-- Table
CREATE TABLE submission_members
(
    created_at    TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta          JSONB                    DEFAULT '{}',

    assignment_id TEXT                   NOT NULL REFERENCES assignments (id),
    enrollment_id TEXT                   NOT NULL REFERENCES enrollments (id),
    submission_id TEXT                   NOT NULL REFERENCES submissions (id),
    role          submission_member_role NOT NULL,

    PRIMARY KEY (assignment_id, enrollment_id) -- composite primary key
);

-- Indexes for `submission_members`
CREATE INDEX idx_submission_members_assignment_id ON submission_members (assignment_id);
CREATE INDEX idx_submission_members_enrollment_id ON submission_members (enrollment_id);
CREATE INDEX idx_submission_members_submission_id ON submission_members (submission_id);
CREATE INDEX idx_submission_members_assignment_id_enrollment_id ON submission_members (assignment_id, submission_id);
