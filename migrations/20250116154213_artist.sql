-- Add migration script here
CREATE TABLE "artist" (
    "artist_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "from" TEXT NOT NULL
);

-- CREATE TABLE "venue" (
--     "venue_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
--     "name" TEXT NOT NULL,
--     "city" TEXT NOT NULL
-- );

-- CREATE TABLE "gig" (
--     "artist_id" INTEGER NOT NULL REFERENCES "artist"("artist_id"),
--     "venue_id" INTEGER NOT NULL REFERENCES "venue"("venue_id"),
--     "date" TEXT NOT NULL,
--     "act" INTEGER NOT NULL

--     PRIMARY KEY("artist_id", "venue_id")
-- );
