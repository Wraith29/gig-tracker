-- Add migration script here

CREATE TABLE "new_gig" (
    "artist_id" INTEGER NOT NULL,
    "venue_id" INTEGER NOT NULL,
    "date" TEXT NOT NULL,
    "act" INTEGER NOT NULL,

    PRIMARY KEY ("artist_id", "venue_id", "date"),
    FOREIGN KEY ("artist_id") REFERENCES "artist" ("artist_id"),
    FOREIGN KEY ("venue_id") REFERENCES "venue" ("venue_id")
);

INSERT INTO "new_gig"
SELECT * FROM "gig";

DROP TABLE "gig";

ALTER TABLE "new_gig" RENAME TO "gig";
