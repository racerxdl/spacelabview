package spacelab

type Player struct {
	Id       string
	Name     string
	Faction  string
	SteamId  string
	IsOnline bool
	X, Y, Z  float64
	Position Vector3
	Rotation Quaternion
}

func (g Player) DistanceTo(g2 Player) float64 {
	a := g
	b := g2

	xD := b.X - a.X
	yD := b.Y - a.Y
	zD := b.Z - a.Z

	return xD*xD + yD*yD + zD*zD
}

func (g Player) AtOrigin() bool {
	return g.X == 0 && g.Y == 0 && g.Z == 0
}

type PlayerUpdate struct {
	Player    Player
	IsNew     bool
	IsDeleted bool
}
