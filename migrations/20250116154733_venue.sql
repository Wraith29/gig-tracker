-- Add migration script here
CREATE TABLE "venue" (
    "venue_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "city" TEXT NOT NULL
);
