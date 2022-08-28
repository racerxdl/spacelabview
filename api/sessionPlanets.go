package api

import "encoding/json"

type Planet struct {
	DisplayName string
	EntityId    int64
	Position    Vec3D
}

const sessionPlanets = "/v1/session/planets"

type sessionPlanetsResponseData struct {
	Planets []Planet
}

type sessionPlanetsResponse struct {
	Data sessionPlanetsResponseData `json:"data"`
	Meta Meta                       `json:"meta"`
}

func (s *SpaceAPI) SessionPlanets() ([]Planet, error) {
	res, err := s.Get(sessionPlanets, nil)
	if err != nil {
		return nil, err
	}

	g := sessionPlanetsResponse{}
	err = json.Unmarshal([]byte(res), &g)
	return g.Data.Planets, err
}
