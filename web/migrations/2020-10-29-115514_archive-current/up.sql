CREATE TABLE IF NOT EXISTS archive AS
    (SELECT *
     FROM crawled);


DELETE
FROM crawled;
