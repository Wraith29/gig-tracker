package data

import (
	"strconv"

	"github.com/charmbracelet/bubbles/table"
	"gorm.io/gorm"
)

type Artist struct {
	ArtistId uint `gorm:"primaryKey"`
	Name     string
	From     string
	Gigs     []Gig `gorm:"foreignKey:ArtistId;constraint:OnDelete:CASCADE"`
}

func (a Artist) ToRow() table.Row {
	return table.Row{
		strconv.FormatUint(uint64(a.ArtistId), 10),
		a.Name,
		a.From,
	}
}

func GetArtistTable(db *gorm.DB) (table.Model, error) {
	artists := make([]Artist, 0)
	if result := db.Find(&artists); result.Error != nil {
		return table.Model{}, result.Error
	}

	artistRows := make([]table.Row, len(artists))
	for idx, artist := range artists {
		artistRows[idx] = artist.ToRow()
	}

	artistTable := table.New(
		table.WithColumns([]table.Column{
			{Title: "Artist Id", Width: 10},
			{Title: "Name", Width: 20},
			{Title: "From", Width: 20},
		}),
		table.WithRows(artistRows),
		table.WithWidth(50),
		table.WithHeight(10),
	)

	return artistTable, nil
}
