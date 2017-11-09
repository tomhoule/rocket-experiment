ALTER TABLE editions ADD COLUMN slug varchar(200) NOT NULL UNIQUE;
CREATE UNIQUE INDEX editions_slug_idx ON editions(slug);

CREATE TABLE fragments (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  edition_id UUID REFERENCES editions(id) ON DELETE CASCADE NOT NULL,
  fragment_path TEXT NOT NULL,
  value TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  CONSTRAINT fragment_uniqueness UNIQUE(edition_id, fragment_path)
);

SELECT diesel_manage_updated_at('fragments');
