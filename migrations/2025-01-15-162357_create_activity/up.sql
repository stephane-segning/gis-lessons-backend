-- Enums for `activities`
CREATE TYPE activity_type AS ENUM ('create', 'delete', 'update');
CREATE TYPE activity_entity_type AS ENUM ('student', 'instructor', 'course', 'enrollment', 'module', 'assignment', 'submission', 'activity', 'comment');

-- Table
CREATE TABLE activities
(
    id          TEXT PRIMARY KEY,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    meta        JSONB                    DEFAULT '{}',

    user_id     TEXT                 NOT NULL,
    entity_id   TEXT                 NOT NULL,
    entity_type activity_entity_type NOT NULL,
    content     VARCHAR              NOT NULL,
    action_type activity_type        NOT NULL
);

-- Indexes for `activities`
CREATE INDEX idx_activities_user_id ON activities (user_id);
CREATE INDEX idx_activities_entity_id ON activities (entity_id);
CREATE INDEX idx_activities_entity_type ON activities (entity_type);
CREATE INDEX idx_activities_action_type ON activities (action_type);
