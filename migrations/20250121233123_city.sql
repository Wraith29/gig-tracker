-- Add migration script here
DROP TABLE gig;
DROP TABLE artist;
DROP TABLE venue;

CREATE TABLE "city" (
    "city_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL
);

CREATE TABLE "artist" (
    "artist_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "city_id" INTEGER NOT NULL,

    FOREIGN KEY ("city_id") REFERENCES "city"("city_id")
);

CREATE TABLE "venue" (
    "venue_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "city_id" INTEGER NOT NULL,

    FOREIGN KEY ("city_id") REFERENCES "city"("city_id")
);

CREATE TABLE "gig" (
    "artist_id" INTEGER NOT NULL,
    "venue_id" INTEGER NOT NULL,
    "date" TEXT NOT NULL,
    "act" INTEGER NOT NULL,

    PRIMARY KEY("artist_id", "venue_id"),
    FOREIGN KEY ("artist_id") REFERENCES "artist"("artist_id"),
    FOREIGN KEY ("venue_id") REFERENCES "venue"("venue_id")
);
