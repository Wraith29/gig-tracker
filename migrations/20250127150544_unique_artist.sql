-- Add migration script here

CREATE TABLE "new_artist" (
    "artist_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL UNIQUE,
    "city_id" INTEGER NOT NULL,

    FOREIGN KEY ("city_id") REFERENCES "city"("city_id")
);

DELETE FROM "artist" WHERE "artist_id" = 5;

INSERT INTO "new_artist" SELECT * FROM "artist";

DROP TABLE "gig";
DROP TABLE "artist";

ALTER TABLE "new_artist"
RENAME TO "artist";


CREATE TABLE "gig" (
    "artist_id" INTEGER NOT NULL,
    "venue_id" INTEGER NOT NULL,
    "date" TEXT NOT NULL,
    "act" INTEGER NOT NULL,

    PRIMARY KEY("artist_id", "venue_id"),
    FOREIGN KEY ("artist_id") REFERENCES "artist"("artist_id"),
    FOREIGN KEY ("venue_id") REFERENCES "venue"("venue_id")
);
