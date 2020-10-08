-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE websites
(
  id UUID NOT NULL DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL, 
  created_at timestamp,
  updated_at timestamp,
  PRIMARY KEY ( id )
)
