package style

import (
	"github.com/charmbracelet/bubbles/table"
	lg "github.com/charmbracelet/lipgloss"
)

var (
	Title = lg.NewStyle().Bold(true).Underline(true)

	Unfocused = lg.NewStyle().
			BorderStyle(lg.NormalBorder()).
			BorderForeground(lg.Color("#f00"))

	Focused = lg.NewStyle().
		BorderStyle(lg.NormalBorder()).
		BorderForeground(lg.Color("#00f"))

	Table = table.DefaultStyles()

	Form = lg.NewStyle().
		BorderStyle(lg.NormalBorder()).
		BorderForeground(lg.Color("#0f0"))
)
