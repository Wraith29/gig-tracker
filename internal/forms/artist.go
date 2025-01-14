package forms

import (
	"gig-tracker/internal/style"
	"strings"

	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
)

type ArtistFormModel struct {
	name textinput.Model
	from textinput.Model
}

func NewArtistFormModel() ArtistFormModel {
	return ArtistFormModel{
		name: textinput.New(),
		from: textinput.New(),
	}
}

func (a *ArtistFormModel) updateInputs(msg tea.Msg) (tea.Model, tea.Cmd) {
	var nameCmd, fromCmd tea.Cmd

	a.name, nameCmd = a.name.Update(msg)
	a.from, fromCmd = a.from.Update(msg)

	return a, tea.Batch(nameCmd, fromCmd)
}

func (a *ArtistFormModel) Clear() {
	a.name.Reset()
	a.from.Reset()
}

func (a *ArtistFormModel) Init() tea.Cmd {
	return nil
}

func (a *ArtistFormModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	return a.updateInputs(msg)
}

func (a *ArtistFormModel) View() string {
	form := strings.Builder{}

	form.WriteString(style.Form.Render(a.name.View()) + "\n")
	form.WriteString(style.Form.Render(a.from.View()) + "\n")

	return form.String()
}
