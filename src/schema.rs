// @generated automatically by Diesel CLI.

diesel::table! {
    edit_drafts (user_id, guild_id, edit_hash) {
        user_id -> Integer,
        guild_id -> Integer,
        edit_hash -> Text,
        target_ver -> Text,
        offset_x -> Integer,
        offset_y -> Integer,
        credit_cost -> Integer,
        flags -> Integer,
    }
}

diesel::table! {
    guild_walls (guild_id) {
        guild_id -> Nullable<Integer>,
        active_wall_ver -> Text,
        last_scaled_tstamp -> Text,
        credit_gain -> Integer,
        credit_cap -> Integer,
        flags -> Integer,
    }
}

diesel::table! {
    member_accounts (user_id, guild_id) {
        user_id -> Integer,
        guild_id -> Integer,
        credits -> Integer,
        tstamp -> Integer,
        last_contribution_ver -> Nullable<Text>,
        flags -> Integer,
    }
}

diesel::table! {
    user_walls (user_id) {
        user_id -> Nullable<Integer>,
        active_wall_ver -> Text,
        adopted_guild_id -> Nullable<Integer>,
        flags -> Integer,
    }
}

diesel::table! {
    wall_versions (owner_id, owner_kind, wall_ver) {
        owner_id -> Integer,
        owner_kind -> Integer,
        wall_ver -> Text,
        tstamp -> Integer,
        width -> Integer,
        height -> Integer,
        contributors -> Text,
        prev_id -> Nullable<Text>,
        flags -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    edit_drafts,
    guild_walls,
    member_accounts,
    user_walls,
    wall_versions,
);
