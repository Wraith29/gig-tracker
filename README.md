# Gig Tracker

TUI application to allow the tracking of various gig stats.

The Left Column is all of the data available, and a form at the bottom to add new data in.

The Right Column is different graph views. I need to come up with a nice way to make either lots of data views available, and selectable,
or allow the user to select different aspects of datasets to view custom graphs.

Right now, the Graph Column is going to be hard coded with data that I want to see

## Setup

Create a `.env` file with the following values:
```sh
DATABASE_URL="sqlite:/path/to/my/db-file.db"
```

At some point I will make this go to a command place (Probably something like .local/gig-tracker on Linux), but for now it's hardcoded coz I'm lazy.

Once that is done, run:
```
cargo install sqlx-cli # Installs the CLI tool we are about to use

sqlx db create # Creates the .db file at the location pointed to in your .env

sqlx migrate run # Runs the migration scripts, which will create the tables necessary for the app to run
```
