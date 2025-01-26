CREATE TYPE submission_type AS ENUM ('draft', 'submitted');

-- Table
CREATE TABLE submissions
(
    id             TEXT PRIMARY KEY,
    created_at     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta           JSONB                    DEFAULT '{}',

    assignment_id  TEXT            NOT NULL REFERENCES assignments (id),
    date_submitted TIMESTAMP WITH TIME ZONE,
    status       submission_type NOT NULL,
    content        TEXT            NOT NULL
);

-- Indexes for `submissions`
CREATE INDEX idx_submissions_status ON submissions (status);
CREATE INDEX idx_submissions_assignment_id ON submissions (assignment_id);
CREATE INDEX idx_submissions_assignment_id_status ON submissions (assignment_id, status);
