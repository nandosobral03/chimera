-- Your SQL goes here

CREATE INDEX guest_day_stats_day_idx ON guest_day_stats(day);
CREATE INDEX guest_day_stats_guest_idx ON guest_day_stats(guest_id);


CREATE INDEX user_day_stats_day_idx ON user_day_stats(day);
CREATE INDEX user_day_stats_user_idx ON user_day_stats(user_id);

CREATE INDEX day_stats_idx ON day_stats(day);

CREATE INDEX guests_idx ON guests(id);

CREATE INDEX user_stats_idx ON user_stats(user_id);