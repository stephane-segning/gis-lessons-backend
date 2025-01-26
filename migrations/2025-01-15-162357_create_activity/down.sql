-- This file should undo anything in `up.sql`

DROP INDEX idx_activities_action_type ON activities;
DROP INDEX idx_activities_entity_type ON activities;
DROP INDEX idx_activities_entity_id ON activities;
DROP INDEX idx_activities_user_id ON activities;

DROP TABLE IF EXISTS activities RESTRICT;