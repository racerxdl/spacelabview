package spacelab

import "fmt"

type Grid struct {
	Id            string
	Name          string
	Owner         string
	Faction       string
	FactionTag    string
	Blocks        int
	IsPowered     bool
	GridSize      float64
	IsStatic      bool
	IsParked      bool
	ParentId      string
	RelGroupId    int
	RelGroupCount int
	PCU           int
	X, Y, Z       float64
}

func (g Grid) String() string {
	return fmt.Sprintf("%s - Blocks: %d - Owner: %q - PCU: %d", g.Name, g.Blocks, g.Owner, g.PCU)
}

func (g Grid) DistanceToGrid(g2 Grid) float64 {
	a := g
	b := g2

	xD := b.X - a.X
	yD := b.Y - a.Y
	zD := b.Z - a.Z

	return xD*xD + yD*yD + zD*zD
}

type GridUpdate struct {
	Grid      Grid
	IsNew     bool
	IsDeleted bool
}
