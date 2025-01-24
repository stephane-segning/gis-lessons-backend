use diesel::table;

table! {
    activities {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        user_id -> Text,
        entity_id -> Text,
        entity_type -> VarChar,
        content -> VarChar,
        action_type -> VarChar
    }
}

table! {
    courses {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        name -> Text,
        slug -> Text,
        description -> Text,
    }
}

table! {
    assignments {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        lesson_id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        due_date -> Timestamp,
    }
}

table! {
    comments {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        user_id -> Nullable<Text>,
        owner_id -> Nullable<Text>,
        content -> Nullable<Text>,
        r#type -> Nullable<Text>,
    }
}

table! {
    enrollments {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        user_id -> Text,
        course_id -> Text,
        enrollment_type -> Text
    }
}

table! {
    lessons {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        module_id -> Text,
        title -> Text,
        description -> Nullable<Text>
    }
}

table! {
    lesson_blocks {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        r#type -> Text,
        data -> Jsonb
    }
}

table! {
    modules {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        course_id -> Text,
        title -> Text,
        description -> Text
    }
}

table! {
    submissions {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,
        
        assignment_id -> Text,
        date_submitted -> Nullable<Timestamp>,
        status -> Text,
        content -> Text,
    }
}

table! {
    submission_member (assignment_id, enrollment_id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,
        
        assignment_id -> Text,
        enrollment_id -> Text,
        
        submission_id -> Text,
        role -> Text,
    }
}
