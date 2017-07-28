CREATE TABLE editions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    editor TEXT NOT NULL,
    year INTEGER NOT NULL,
    language_code TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('editions');
