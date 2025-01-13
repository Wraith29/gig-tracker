package data

import (
	"strconv"

	"github.com/charmbracelet/bubbles/table"
	"gorm.io/gorm"
)

type Venue struct {
	VenueId uint `gorm:"primaryKey"`
	Name    string
	City    string
	Gigs    []Gig `gorm:"foreignKey:VenueId;constraint:OnDelete:CASCADE"`
}

func (v Venue) ToRow() table.Row {
	return table.Row{
		strconv.FormatUint(uint64(v.VenueId), 10),
		v.Name,
		v.City,
	}
}

func GetVenueTable(db *gorm.DB) (table.Model, error) {
	venues := make([]Venue, 0)
	if result := db.Find(&venues); result.Error != nil {
		return table.Model{}, result.Error
	}

	venueRows := make([]table.Row, len(venues))
	for idx, venue := range venues {
		venueRows[idx] = venue.ToRow()
	}

	venueTable := table.New(
		table.WithColumns([]table.Column{
			{Title: "Venue Id", Width: 10},
			{Title: "Name", Width: 20},
			{Title: "City", Width: 20},
		}),
		table.WithRows(venueRows),
		table.WithWidth(50),
		table.WithHeight(10),
	)

	return venueTable, nil
}
