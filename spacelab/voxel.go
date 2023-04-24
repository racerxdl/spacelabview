package spacelab

type HillParams struct {
	Min float64 `json:"Item1"`
	Max float64 `json:"Item2"`
}

type Voxel struct {
	Id                 string
	Name               string
	DebugName          string
	X, Y, Z            float64
	Size               float64
	HasAtmosphere      bool
	AtmosphereAltitude float64
	HillParameters     HillParams
}
