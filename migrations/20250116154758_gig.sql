-- Add migration script here
CREATE TABLE "gig" (
    "artist_id" INTEGER NOT NULL,
    "venue_id" INTEGER NOT NULL,
    "date" TEXT NOT NULL,
    "act" INTEGER NOT NULL,

    PRIMARY KEY("artist_id", "venue_id"),
    FOREIGN KEY ("artist_id") REFERENCES "artist"("artist_id"),
    FOREIGN KEY ("venue_id") REFERENCES "venue"("venue_id")
);
