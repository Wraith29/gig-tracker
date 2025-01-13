package data

import (
	"log"
	"strings"

	"github.com/charmbracelet/bubbles/table"
	tea "github.com/charmbracelet/bubbletea"
	lg "github.com/charmbracelet/lipgloss"
	"gorm.io/gorm"
)

var (
	componentStyle = lg.NewStyle().
			BorderStyle(lg.NormalBorder()).
			BorderForeground(lg.Color("#f00"))

	tableStyle = table.DefaultStyles()
)

var style = lg.NewStyle().BorderStyle(lg.NormalBorder()).BorderForeground(lg.Color("#00ff00"))

type App struct {
	artistTable *table.Model
	venueTable  *table.Model
	gigTable    *table.Model
}

func NewApp(db *gorm.DB) (*App, error) {
	tableStyle.Header = tableStyle.Header.
		BorderStyle(lg.NormalBorder()).
		BorderBottom(true).
		BorderForeground(lg.Color("#ff0000"))

	artistTable, err := GetArtistTable(db)
	if err != nil {
		return nil, err
	}
	artistTable.SetStyles(tableStyle)

	venueTable, err := GetVenueTable(db)
	if err != nil {
		return nil, err
	}
	venueTable.SetStyles(tableStyle)

	gigTable, err := GetGigTable(db)
	if err != nil {
		return nil, err
	}
	gigTable.SetStyles(tableStyle)

	return &App{
		artistTable: artistTable,
		venueTable:  venueTable,
		gigTable:    gigTable,
	}, nil
}

func (a App) Init() tea.Cmd {
	return nil
}

func (a App) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch message := msg.(type) {
	case tea.KeyMsg:
		log.Printf("Key Pressed: %s\n", message.String())

		if message.String() == "q" || message.String() == "ctrl+c" {
			return a, tea.Quit
		}
	}

	return a, nil
}

func (a App) View() string {
	output := strings.Builder{}

	output.WriteString("Artists:\n")
	output.WriteString(componentStyle.Render(a.artistTable.View()))
	output.WriteString("\n\n")

	output.WriteString("Venues:\n")
	output.WriteString(componentStyle.Render(a.venueTable.View()))
	output.WriteString("\n\n")

	output.WriteString("Gigs:\n")
	output.WriteString(componentStyle.Render(a.gigTable.View()))
	output.WriteString("\n\n")

	return output.String()
}
