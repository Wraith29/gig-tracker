package data

import (
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

func InitDb() (*gorm.DB, error) {
	db, err := gorm.Open(sqlite.Open("gigs.db"))
	if err != nil {
		return nil, err
	}

	if err := db.AutoMigrate(&Artist{}); err != nil {
		return nil, err
	}

	if err := db.AutoMigrate(&Venue{}); err != nil {
		return nil, err
	}

	if err := db.AutoMigrate(&Gig{}); err != nil {
		return nil, err
	}

	return db, nil
}
