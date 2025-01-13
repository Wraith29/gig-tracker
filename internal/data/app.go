package data

import (
	"strings"

	"github.com/charmbracelet/bubbles/help"
	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/table"
	tea "github.com/charmbracelet/bubbletea"
	lg "github.com/charmbracelet/lipgloss"
	"gorm.io/gorm"
)

var (
	titleStyle = lg.NewStyle().Bold(true).Underline(true)

	unfocusedStyle = lg.NewStyle().
			BorderStyle(lg.NormalBorder()).
			BorderForeground(lg.Color("#f00"))

	focusedStyle = lg.NewStyle().
			BorderStyle(lg.NormalBorder()).
			BorderForeground(lg.Color("#00f"))

	tableStyle = table.DefaultStyles()
)

type focusedApp int

const (
	faArtist focusedApp = iota
	faVenue
	faGig
)

func (f focusedApp) up() focusedApp {
	switch f {
	case faArtist:
		return faGig
	case faVenue:
		return faArtist
	case faGig:
		return faVenue
	}

	return faArtist
}

func (f focusedApp) down() focusedApp {
	switch f {
	case faArtist:
		return faVenue
	case faVenue:
		return faGig
	case faGig:
		return faArtist
	}

	return faArtist
}

type keyMap struct {
	Quit key.Binding
	Up   key.Binding
	Down key.Binding
}

func (k keyMap) ShortHelp() []key.Binding {
	return []key.Binding{k.Quit, k.Up, k.Down}
}

func (k keyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{
		{k.Up, k.Down},
		{k.Quit},
	}
}

var keys = keyMap{
	Quit: key.NewBinding(
		key.WithKeys("q", "ctrl+c"),
		key.WithHelp("q/ctrl+c", "Quit"),
	),
	Up: key.NewBinding(
		key.WithKeys("K"),
		key.WithHelp("K", "Move Up"),
	),
	Down: key.NewBinding(
		key.WithKeys("J"),
		key.WithHelp("J", "Move Down"),
	),
}

type App struct {
	window *tea.WindowSizeMsg

	artistTable table.Model
	venueTable  table.Model
	gigTable    table.Model

	helpModel *help.Model
	keys      keyMap

	focused focusedApp
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
	artistTable.Focus()
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

	helpModel := help.New()

	return &App{
		artistTable: artistTable,
		venueTable:  venueTable,
		gigTable:    gigTable,
		helpModel:   &helpModel,
		keys:        keys,
		focused:     faArtist,
	}, nil
}

func (a *App) changeFocus(newFocus focusedApp) {
	a.focused = newFocus

	a.artistTable.Blur()
	a.venueTable.Blur()
	a.gigTable.Blur()

	switch a.focused {
	case faArtist:
		a.artistTable.Focus()
	case faVenue:
		a.venueTable.Focus()
	case faGig:
		a.gigTable.Focus()
	}
}

func (a *App) resizeTables() {
	width := a.window.Width / 2
	height := (a.window.Height - 9) / 3

	a.artistTable.SetWidth(width)
	a.venueTable.SetWidth(width)
	a.gigTable.SetWidth(width)

	a.artistTable.SetHeight(height)
	a.venueTable.SetHeight(height)
	a.gigTable.SetHeight(height)
}

func (a *App) updateTables(msg tea.Msg) (tea.Model, tea.Cmd) {
	var artistCmd, venueCmd, gigCmd tea.Cmd

	a.artistTable, artistCmd = a.artistTable.Update(msg)
	a.venueTable, venueCmd = a.venueTable.Update(msg)
	a.gigTable, gigCmd = a.gigTable.Update(msg)

	return a, tea.Batch(artistCmd, venueCmd, gigCmd)
}

func (a App) Init() tea.Cmd {
	return nil
}

func (a App) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch message := msg.(type) {
	case tea.WindowSizeMsg:
		a.window = &message
		a.resizeTables()
	case tea.KeyMsg:
		switch {
		case key.Matches(message, a.keys.Quit):
			return a, tea.Quit
		case key.Matches(message, a.keys.Up):
			a.changeFocus(a.focused.up())
		case key.Matches(message, a.keys.Down):
			a.changeFocus(a.focused.down())
		}

	}

	return a.updateTables(msg)
}

func (a App) View() string {
	output := strings.Builder{}

	output.WriteString(titleStyle.Render("Gig Tracker") + "\n")

	if a.artistTable.Focused() {
		output.WriteString(focusedStyle.Render(a.artistTable.View()) + "\n" + a.artistTable.HelpView() + "\n")
	} else {
		output.WriteString(unfocusedStyle.Render(a.artistTable.View()) + "\n")
	}

	if a.venueTable.Focused() {
		output.WriteString(focusedStyle.Render(a.venueTable.View()) + "\n" + a.venueTable.HelpView() + "\n")
	} else {
		output.WriteString(unfocusedStyle.Render(a.venueTable.View()) + "\n")
	}

	if a.gigTable.Focused() {
		output.WriteString(focusedStyle.Render(a.gigTable.View()) + "\n" + a.gigTable.HelpView() + "\n")
	} else {
		output.WriteString(unfocusedStyle.Render(a.gigTable.View()) + "\n")
	}

	output.WriteString(a.helpModel.View(a.keys))

	return output.String()
}
