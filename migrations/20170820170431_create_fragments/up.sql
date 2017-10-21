ALTER TABLE editions ADD COLUMN slug varchar(200) NOT NULL UNIQUE;
CREATE UNIQUE INDEX editions_slug_idx ON editions(slug);

CREATE TABLE fragments (
    edition_slug varchar(200) REFERENCES editions(slug) NOT NULL,
    fragment_path TEXT NOT NULL,
    value TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT fragment_uniqueness PRIMARY KEY(edition_slug, fragment_path)
);

SELECT diesel_manage_updated_at('fragments');
