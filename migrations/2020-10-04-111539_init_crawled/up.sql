CREATE TABLE crawled (
    id SERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    counts JSON NOT NULL,
    page_hash VARCHAR(64) NOT NULL UNIQUE,
    url VARCHAR NOT NULL
);
