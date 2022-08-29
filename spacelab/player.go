package spacelab

type Player struct {
	Id       string
	Name     string
	Faction  string
	SteamId  string
	IsOnline bool
	X, Y, Z  float64
}
