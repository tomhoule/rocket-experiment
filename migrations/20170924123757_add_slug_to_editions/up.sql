ALTER TABLE editions ADD COLUMN slug varchar(200) NOT NULL UNIQUE;
CREATE UNIQUE INDEX editions_slug_idx ON editions(slug);
