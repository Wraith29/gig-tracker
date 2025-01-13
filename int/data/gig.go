package data

import (
	"strconv"
	"time"

	"github.com/charmbracelet/bubbles/table"
	"gorm.io/gorm"
)

type Act uint

const (
	actMain Act = iota
	actSupport
	actSharedHeadliner
)

func (a Act) String() string {
	switch a {
	case actMain:
		return "Main Act"
	case actSupport:
		return "Support Act"
	case actSharedHeadliner:
		return "Shared Headliner"
	}

	return "Unknown"
}

type Gig struct {
	ArtistId uint `gorm:"primaryKey;autoIncrement:false"`
	VenueId  uint `gorm:"primaryKey;autoIncrement:false"`
	Date     time.Time
	Act      Act
}

func (g Gig) ToRow() table.Row {
	return table.Row{
		strconv.FormatUint(uint64(g.ArtistId), 10),
		strconv.FormatUint(uint64(g.VenueId), 10),
		g.Date.Format("2006/01/02"),
		g.Act.String(),
	}
}

func GetGigTable(db *gorm.DB) (*table.Model, error) {
	gigs := make([]Gig, 0)
	if result := db.Find(&gigs); result.Error != nil {
		return nil, result.Error
	}

	gigRows := make([]table.Row, len(gigs))
	for idx, gig := range gigs {
		gigRows[idx] = gig.ToRow()
	}

	gigTable := table.New(
		table.WithColumns([]table.Column{
			{Title: "Artist Id", Width: 10},
			{Title: "Venue Id", Width: 10},
			{Title: "Date", Width: 10},
			{Title: "Act", Width: 20},
		}),
		table.WithRows(gigRows),
		table.WithWidth(50),
		table.WithHeight(10),
	)

	return &gigTable, nil
}
