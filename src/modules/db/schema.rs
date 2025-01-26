use diesel::table;

table! {
    accounts {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        sub -> Text,
        name -> Text
    }
}

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

diesel::joinable!(activities -> accounts (user_id));

diesel::allow_tables_to_appear_in_same_query!(activities, accounts);

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

diesel::joinable!(assignments -> lessons (lesson_id));

diesel::allow_tables_to_appear_in_same_query!(lessons, assignments);

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

diesel::joinable!(comments -> accounts (user_id));

diesel::allow_tables_to_appear_in_same_query!(comments, accounts);

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

diesel::joinable!(enrollments -> accounts (user_id));

diesel::allow_tables_to_appear_in_same_query!(enrollments, accounts);

table! {
    lessons {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        module_id -> Text,
        title -> Text,
        content -> Jsonb,
        description -> Nullable<Text>
    }
}

diesel::joinable!(lessons -> modules (module_id));

diesel::allow_tables_to_appear_in_same_query!(modules, lessons);

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

diesel::joinable!(modules -> courses (course_id));

diesel::allow_tables_to_appear_in_same_query!(courses, modules);

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

diesel::joinable!(submissions -> assignments (assignment_id));

diesel::allow_tables_to_appear_in_same_query!(assignments, submissions);

table! {
    submission_members (assignment_id, enrollment_id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        assignment_id -> Text,
        enrollment_id -> Text,

        submission_id -> Text,
        role -> Text,
    }
}

diesel::joinable!(submission_members -> assignments (assignment_id));
diesel::joinable!(submission_members -> enrollments (enrollment_id));
diesel::joinable!(submission_members -> submissions (submission_id));

diesel::allow_tables_to_appear_in_same_query!(assignments, submission_members);
diesel::allow_tables_to_appear_in_same_query!(enrollments, submission_members);
diesel::allow_tables_to_appear_in_same_query!(submissions, submission_members);
