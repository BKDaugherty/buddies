CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  password VARCHAR(64) NOT NULL,
  user_id VARCHAR NOT NULL,
  create_timestamp VARCHAR NOT NULL
)	     
