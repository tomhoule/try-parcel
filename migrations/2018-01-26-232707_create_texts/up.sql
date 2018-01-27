CREATE TABLE texts (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  title TEXT NOT NULL,
  slug VARCHAR(140) UNIQUE NOT NULL,
  authors VARCHAR(300) NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE schemas (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  text_id UUID NOT NULL REFERENCES texts,
  paths TEXT[] NOT NULL DEFAULT '{ index }',
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE fragments (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  schema_path TEXT NOT NULL,
  text_id UUID REFERENCES texts,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('texts');
