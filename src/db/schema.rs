table! {
    activity_details (id) {
        id -> Nullable<Integer>,
        a_uuid -> Text,
        name -> Nullable<Text>,
    }
}

table! {
    activity_location_info (id) {
        id -> Nullable<Integer>,
        a_uuid -> Text,
        name -> Nullable<Text>,
        url -> Nullable<Text>,
        source -> Nullable<Text>,
    }
}

table! {
    activity_products (id) {
        id -> Nullable<Integer>,
        a_uuid -> Text,
        name -> Nullable<Text>,
    }
}

table! {
    activity_sub_title (id) {
        id -> Nullable<Integer>,
        a_uuid -> Text,
        name -> Nullable<Text>,
        url -> Nullable<Text>,
    }
}

table! {
    google_my_activity (uuid) {
        uuid -> Text,
        header -> Text,
        title -> Text,
        title_url -> Nullable<Text>,
        time -> Text,
    }
}

table! {
    location_history (id) {
        id -> Nullable<Integer>,
        activity -> Nullable<Text>,
        timestamp_msec -> BigInt,
        accuracy -> Nullable<Integer>,
        verticalaccuracy -> Nullable<Integer>,
        altitude -> Nullable<Integer>,
        lat -> Float,
        lng -> Float,
    }
}

joinable!(activity_details -> google_my_activity (a_uuid));
joinable!(activity_location_info -> google_my_activity (a_uuid));
joinable!(activity_products -> google_my_activity (a_uuid));
joinable!(activity_sub_title -> google_my_activity (a_uuid));

allow_tables_to_appear_in_same_query!(
    activity_details,
    activity_location_info,
    activity_products,
    activity_sub_title,
    google_my_activity,
    location_history,
);
