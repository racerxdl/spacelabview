package spacelab

import "encoding/json"

const (
	infoEndpoint         = "/info"
	infoVoxelsEndpoint   = infoEndpoint + "/voxels"
	infoFactionsEndpoint = infoEndpoint + "/factions"
	infoPlayersEndpoint  = infoEndpoint + "/players"
	infoGridsEndpoint    = infoEndpoint + "/grids"
)

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
