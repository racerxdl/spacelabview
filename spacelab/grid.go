package spacelab

import (
	"fmt"

	"github.com/racerxdl/spacelabview/spaceproto"
)

type GridBlock struct {
	GridPosition Vector3 `json:"GridPosition"`
	Position     Vector3 `json:"Position"`
	MaxHealth    float64 `json:"MaxHealth"`
	Health       float64 `json:"Health"`
	BlockType    string  `json:"BlockType"`
}

func (gb GridBlock) ToProto() *spaceproto.GridBlock {
	return &spaceproto.GridBlock{
		GridPosition: gb.GridPosition.ToProto(),
		Position:     gb.Position.ToProto(),
		MaxHealth:    gb.MaxHealth,
		Health:       gb.Health,
		BlockType:    gb.BlockType,
	}
}

func (gb GridBlock) String() string {
	return fmt.Sprintf("%s - Health: %f/%f (%f,%f,%f)", gb.BlockType, gb.Health, gb.MaxHealth, gb.Position.X, gb.Position.Y, gb.Position.Z)
}

type GridGroup struct {
	Grids   []Grid
	Owner   string
	Faction string
	Tag     string
	Blocks  int
}

func (gg GridGroup) ToProto() *spaceproto.GridGroup {
	gridGroup := &spaceproto.GridGroup{
		Owner:      gg.Owner,
		Faction:    gg.Faction,
		FactionTag: gg.Tag,
		Blocks:     int32(gg.Blocks),
		Grids:      make([]*spaceproto.Grid, len(gg.Grids)),
	}

	for i, g := range gg.Grids {
		gridGroup.Grids[i] = g.ToProto()
	}

	return gridGroup
}

func (g GridGroup) String() string {
	return fmt.Sprintf("GridGroup(%s) - NumBlocks: %d, NumGrids: %d", g.Owner, g.Blocks, len(g.Grids))
}

type Grid struct {
	Id               string
	Name             string
	Owner            string
	Faction          string
	FactionTag       string
	Blocks           int
	IsPowered        bool
	GridSize         float64
	IsStatic         bool
	IsParked         bool
	ParentId         string
	RelGroupId       int
	RelGroupCount    int
	PCU              int
	X, Y, Z          float64
	Position         Vector3
	Rotation         Quaternion
	LastBlocksUpdate int64
}

func (g Grid) ToProto() *spaceproto.Grid {
	return &spaceproto.Grid{
		Id:               g.Id,
		Name:             g.Name,
		Owner:            g.Owner,
		Faction:          g.Faction,
		FactionTag:       g.FactionTag,
		Blocks:           int32(g.Blocks),
		IsPowered:        g.IsPowered,
		GridSize:         g.GridSize,
		IsStatic:         g.IsStatic,
		IsParked:         g.IsParked,
		ParentId:         g.ParentId,
		RelGroupId:       int32(g.RelGroupId),
		RelGroupCount:    int32(g.RelGroupCount),
		PCU:              int32(g.PCU),
		Position:         g.Position.ToProto(),
		Rotation:         g.Rotation.ToProto(),
		LastBlocksUpdate: g.LastBlocksUpdate,
	}
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

func (gu GridUpdate) ToProto() *spaceproto.GridUpdate {
	return &spaceproto.GridUpdate{
		Grid:      gu.Grid.ToProto(),
		IsNew:     gu.IsNew,
		IsDeleted: gu.IsDeleted,
	}
}
