package spacelab

type Voxel struct {
	Id                 string
	Name               string
	DebugName          string
	X, Y, Z            float64
	Position           Vector3
	Rotation           Quaternion
	Size               float64
	HasAtmosphere      bool
	AtmosphereAltitude float64
	HillParameters     []float64
}
