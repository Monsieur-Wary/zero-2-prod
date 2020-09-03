CREATE TABLE subscriptions(
    id uuid PRIMARY KEY,
    email TEXT NOT NULL UNIQUE CHECK (char_length(email) > 0),
    name TEXT NOT NULL CHECK (char_length(email) > 0),
    subscribed_at timestamptz NOT NULL
);