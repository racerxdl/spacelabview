package api

import (
	"encoding/json"
	"fmt"
)

const sessionGrids = "/v1/session/grids"

type sessionGridsResponseData struct {
	Grids []Grid
}

type sessionGridsResponse struct {
	Data sessionGridsResponseData `json:"data"`
	Meta Meta                     `json:"meta"`
}

func (s *SpaceAPI) SessionGrids() ([]Grid, error) {
	res, err := s.Get(sessionGrids, nil)
	if err != nil {
		return nil, err
	}

	g := sessionGridsResponse{}
	err = json.Unmarshal([]byte(res), &g)
	return g.Data.Grids, err
}

func (s *SpaceAPI) DeleteGrid(gridId uint64) error {
	_, err := s.Delete(fmt.Sprintf("%s/%d", sessionGrids, gridId), nil)
	return err
}
