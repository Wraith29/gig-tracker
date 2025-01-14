package cmd

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
