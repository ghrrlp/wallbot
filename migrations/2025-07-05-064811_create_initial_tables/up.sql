-- Your SQL goes here


CREATE TABLE wall_versions
  ( owner_id INTEGER NOT NULL
  , owner_kind INTEGER NOT NULL
  , wall_ver TEXT NOT NULL

  , tstamp INTEGER NOT NULL
  , width INTEGER NOT NULL
  , height INTEGER NOT NULL
  , contributors TEXT NOT NULL
  , prev_id TEXT

  , flags INTEGER NOT NULL

  , PRIMARY KEY (owner_id, owner_kind, wall_ver)
  );


CREATE TABLE guild_walls
  ( guild_id INTEGER PRIMARY KEY

  , active_wall_ver TEXT NOT NULL
  , last_scaled_tstamp TEXT NOT NULL
  , credit_gain INTEGER NOT NULL
  , credit_cap INTEGER NOT NULL

  , flags INTEGER NOT NULL
  );


CREATE TABLE user_walls
  ( user_id INTEGER PRIMARY KEY

  , active_wall_ver TEXT NOT NULL
  , adopted_guild_id INTEGER

  , flags INTEGER NOT NULL
  );


CREATE TABLE member_accounts
  ( user_id INTEGER NOT NULL
  , guild_id INTEGER NOT NULL

  , credits INTEGER NOT NULL
  , tstamp INTEGER NOT NULL
  , last_contribution_ver TEXT

  , flags INTEGER NOT NULL

  , PRIMARY KEY (user_id, guild_id)
  );


CREATE TABLE edit_drafts
  ( user_id INTEGER NOT NULL
  , guild_id INTEGER NOT NULL
  , edit_hash TEXT NOT NULL

  , target_ver TEXT NOT NULL
  , offset_x INTEGER NOT NULL
  , offset_y INTEGER NOT NULL
  , credit_cost INTEGER NOT NULL

  , flags INTEGER NOT NULL

  , PRIMARY KEY (user_id, guild_id, edit_hash)
  );
