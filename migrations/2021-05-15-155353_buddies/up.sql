CREATE TABLE buddies (
  id SERIAL PRIMARY KEY,
  uuid VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  notes TEXT NOT NULL,	
  last_contacted VARCHAR NOT NULL,
  birthday VARCHAR,
  location VARCHAR,
  create_timestamp VARCHAR NOT NULL,
  last_update_timestamp VARCHAR NOT NULL,
  delete_timestamp VARCHAR,
  user_uuid VARCHAR NOT NULL
)

