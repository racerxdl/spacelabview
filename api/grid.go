package api

import "fmt"

type Grid struct {
	DisplayName      string
	EntityId         uint64
	GridSize         string
	BlocksCount      uint64
	Mass             float64
	Position         Vec3D
	LinearSpeed      float64
	DistanceToPlayer float64
	OwnerSteamId     uint64
	OwnerDisplayName string
	IsPowered        bool
	PCU              uint64
}

func (g Grid) String() string {
	return fmt.Sprintf("%s (%s) - Blocks: %d - Owner: %q", g.DisplayName, g.GridSize, g.BlocksCount, g.OwnerDisplayName)
}

func (g Grid) DistanceToGrid(g2 Grid) float64 {
	a := g.Position
	b := g2.Position

	xD := b.X - a.X
	yD := b.Y - a.Y
	zD := b.Z - a.Z

	return xD*xD + yD*yD + zD*zD
}
