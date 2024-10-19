package spacelab

import "github.com/racerxdl/spacelabview/spaceproto"

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

func (v Voxel) ToProto() *spaceproto.Voxel {
	return &spaceproto.Voxel{
		Id:                 v.Id,
		Name:               v.Name,
		DebugName:          v.DebugName,
		Position:           v.Position.ToProto(),
		Rotation:           v.Rotation.ToProto(),
		Size:               v.Size,
		HasAtmosphere:      v.HasAtmosphere,
		AtmosphereAltitude: v.AtmosphereAltitude,
		HillParameters: &spaceproto.Range{
			Min: v.HillParameters[0],
			Max: v.HillParameters[1],
		},
	}
}
