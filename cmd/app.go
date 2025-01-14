package cmd

import (
	"gig-tracker/internal/style"
	"strings"

	"github.com/charmbracelet/bubbles/help"
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
	"gorm.io/gorm"
)

type Page string

const (
	pageTables Page = "Tables"
	pageForm   Page = "Form"
)

type appKeyMap struct {
	Quit, Add, Cancel key.Binding
}

func (a appKeyMap) ShortHelp() []key.Binding {
	return []key.Binding{a.Quit, a.Add, a.Cancel}
}

func (a appKeyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{{a.Quit, a.Add, a.Cancel}}
}

var appKeys = appKeyMap{
	Quit: key.NewBinding(
		key.WithKeys("q", "ctrl+c"),
		key.WithHelp("q", "Quit"),
	),
	Add: key.NewBinding(
		key.WithKeys("+"),
		key.WithHelp("+", "Add Entry"),
	),
	Cancel: key.NewBinding(
		key.WithKeys(tea.KeyBackspace.String()),
		key.WithHelp("←", "Cancel"),
	),
}

type App struct {
	window      *tea.WindowSizeMsg
	pages       map[Page]tea.Model
	currentPage Page

	keyMap appKeyMap
	help   help.Model
}

func NewApp(db *gorm.DB) (*App, error) {
	tablePage, err := NewTablesPage(db)
	if err != nil {
		return nil, err
	}

	pages := map[Page]tea.Model{
		pageTables: tablePage,
	}

	return &App{
		window:      nil,
		pages:       pages,
		currentPage: pageTables,
		keyMap:      appKeys,
		help:        help.New(),
	}, nil
}

// Nothing should happen on creation of this component
func (app App) Init() tea.Cmd {
	return nil
}

func (app App) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch message := msg.(type) {
	case tea.WindowSizeMsg:
		app.window = &message
	case tea.KeyMsg:
		switch {
		case key.Matches(message, app.keyMap.Quit):
			return app, tea.Quit
		}
	case gotoMsg:
		app.currentPage = Page(message)
	}

	page, cmd := app.pages[app.currentPage].Update(msg)

	app.pages[app.currentPage] = page

	return app, cmd
}

func (app App) View() string {
	view := strings.Builder{}

	view.WriteString(style.Title.Render("Gig Tracker") + "\n")
	view.WriteString(app.pages[app.currentPage].View() + "\n")
	view.WriteString(app.help.ShortHelpView(app.keyMap.ShortHelp()))

	return view.String()
}
