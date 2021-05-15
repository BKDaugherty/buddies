CREATE TABLE interactions (
  id SERIAL PRIMARY KEY,
  uuid VARCHAR NOT NULL,
  notes TEXT NOT NULL,	
  participants TEXT [] NOT NULL,
  date VARCHAR,
  create_timestamp VARCHAR NOT NULL,
  last_update_timestamp VARCHAR NOT NULL,
  delete_timestamp VARCHAR,
  user_uuid VARCHAR NOT NULL
)
