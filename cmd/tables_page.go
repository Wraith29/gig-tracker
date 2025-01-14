package cmd

import (
	"gig-tracker/internal/data"
	"gig-tracker/internal/style"
	"strings"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/table"
	tea "github.com/charmbracelet/bubbletea"
	lg "github.com/charmbracelet/lipgloss"
	"gorm.io/gorm"
)

type tablePageKeyMap struct {
	Up, Down, Goto key.Binding
}

func (t tablePageKeyMap) ShortHelp() []key.Binding {
	return []key.Binding{t.Up, t.Down, t.Goto}
}

func (t tablePageKeyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{{t.Up, t.Down, t.Goto}}
}

var tablePageKeys = tablePageKeyMap{
	Up: key.NewBinding(
		key.WithKeys("K"),
		key.WithHelp("K", "Go Up"),
	),
	Down: key.NewBinding(
		key.WithKeys("J"),
		key.WithHelp("J", "Go Down"),
	),
	Goto: key.NewBinding(
		key.WithKeys("+"),
		key.WithHelp("+", "Create new Entry"),
	),
}

type TablesPage struct {
	window  *tea.WindowSizeMsg
	keyMap  tablePageKeyMap
	focused focusedApp

	tables []table.Model
}

func NewTablesPage(db *gorm.DB) (TablesPage, error) {
	tables := make([]table.Model, 0)

	artistTable, err := data.GetArtistTable(db)
	if err != nil {
		return TablesPage{}, err
	}

	artistTable.Focus()
	artistTable.SetStyles(style.Table)
	tables = append(tables, artistTable)

	venueTable, err := data.GetVenueTable(db)
	if err != nil {
		return TablesPage{}, err
	}

	venueTable.SetStyles(style.Table)
	tables = append(tables, venueTable)

	gigTable, err := data.GetGigTable(db)
	if err != nil {
		return TablesPage{}, err
	}

	gigTable.SetStyles(style.Table)
	tables = append(tables, gigTable)

	return TablesPage{
		window:  nil,
		keyMap:  tablePageKeys,
		focused: faArtist,
		tables:  tables,
	}, nil
}

func (t TablesPage) resize(width, height int) {
	newWidth := width / 2
	newHeight := (height - 9) / 3

	for _, table := range t.tables {
		table.SetWidth(newWidth)
		table.SetHeight(newHeight)
	}
}

func (t TablesPage) Init() tea.Cmd {
	return nil
}

func (t TablesPage) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch message := msg.(type) {
	case tea.WindowSizeMsg:
		t.window = &message
		t.resize(message.Width, message.Height)
	case tea.KeyMsg:
		switch {
		case key.Matches(message, t.keyMap.Goto):
			return t, gotoPage(pageForm)
		}
	}

	return t, nil
}

func (t TablesPage) View() string {
	page := strings.Builder{}

	for _, table := range t.tables {
		var tableStyle lg.Style

		if table.Focused() {
			tableStyle = style.Focused
		} else {
			tableStyle = style.Unfocused
		}

		page.WriteString(tableStyle.Render(table.View()) + "\n" + table.HelpView() + "\n")
	}

	return page.String()
}
