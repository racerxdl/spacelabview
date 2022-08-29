package spacelab

type Voxel struct {
	Id                 string
	Name               string
	DebugName          string
	X, Y, Z            float64
	Size               float64
	HasAtmosphere      bool
	AtmosphereAltitude float64
}
