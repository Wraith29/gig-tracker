package main

import (
	"gig-tracker/internal/data"

	tea "github.com/charmbracelet/bubbletea"
)

func main() {
	logFile, err := tea.LogToFile("logs", "debug")
	if err != nil {
		panic(err)
	}
	defer logFile.Close()

	db, err := data.InitDb()
	if err != nil {
		panic(err)
	}

	model, err := data.NewApp(db)
	if err != nil {
		panic(err)
	}

	program := tea.NewProgram(model, tea.WithAltScreen())
	if _, err := program.Run(); err != nil {
		panic(err)
	}
}
