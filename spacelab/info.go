package spacelab

import "encoding/json"

const (
	infoEndpoint         = "/info"
	infoGlobalEndpoint   = infoEndpoint + "/global"
	infoVoxelsEndpoint   = infoEndpoint + "/voxels"
	infoFactionsEndpoint = infoEndpoint + "/factions"
	infoPlayersEndpoint  = infoEndpoint + "/players"
	infoGridsEndpoint    = infoEndpoint + "/grids"
	infoV2GridsEndpoint  = infoEndpoint + "/v2grids"
)

func (s *API) GlobalInfo() (GlobalInfo, error) {
	res, err := s.Get(infoGlobalEndpoint, nil)
	if err != nil {
		return GlobalInfo{}, err
	}

	g := GlobalInfo{}
	err = json.Unmarshal([]byte(res), &g)
	return g, err
}

func (s *API) Voxels() ([]Voxel, error) {
	res, err := s.Get(infoVoxelsEndpoint, nil)
	if err != nil {
		return nil, err
	}

	g := []Voxel{}
	err = json.Unmarshal([]byte(res), &g)
	return g, err
}

func (s *API) Factions() ([]Faction, error) {
	res, err := s.Get(infoFactionsEndpoint, nil)
	if err != nil {
		return nil, err
	}

	g := []Faction{}
	err = json.Unmarshal([]byte(res), &g)
	return g, err
}

func (s *API) Players() ([]Player, error) {
	res, err := s.Get(infoPlayersEndpoint, nil)
	if err != nil {
		return nil, err
	}

	g := []Player{}
	err = json.Unmarshal([]byte(res), &g)
	return g, err
}

func (s *API) Grids() ([]Grid, error) {
	res, err := s.Get(infoGridsEndpoint, nil)
	if err != nil {
		return nil, err
	}

	g := []Grid{}
	err = json.Unmarshal([]byte(res), &g)
	return g, err
}

func (s *API) GridsV2() ([]GridGroup, error) {
	grids, err := s.Grids()
	if err != nil {
		return nil, err
	}
	g := []GridGroup{}

	for _, grid := range grids {
		found := false
		for i, gg := range g {
			if gg.Owner == grid.Owner {
				gg.Grids = append(gg.Grids, grid)
				gg.Blocks += grid.Blocks
				g[i] = gg
				found = true
				break
			}
		}
		if !found {
			g = append(g, GridGroup{
				Owner:  grid.Owner,
				Tag:    grid.FactionTag,
				Blocks: grid.Blocks,
				Grids:  []Grid{grid},
			})
		}
	}

	return g, err
}
