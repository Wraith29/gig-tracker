package cmd

import (
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
)

type keyMap struct {
	Quit key.Binding
	Up   key.Binding
	Down key.Binding
	Add  key.Binding
	Back key.Binding
}

func (k keyMap) ShortHelp() []key.Binding {
	return []key.Binding{k.Quit, k.Up, k.Down, k.Add, k.Back}
}

func (k keyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{
		{k.Up, k.Down},
		{k.Quit, k.Add, k.Back},
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
	Add: key.NewBinding(
		key.WithKeys("+"),
		key.WithHelp("+", "Create new row"),
	),
	Back: key.NewBinding(
		key.WithKeys(tea.KeyBackspace.String()),
		key.WithHelp("←", "Go Back"),
	),
}
