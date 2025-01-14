package cmd

import tea "github.com/charmbracelet/bubbletea"

type gotoMsg Page

func gotoPage(page Page) tea.Cmd {
	return func() tea.Msg {
		return gotoMsg(page)
	}
}
