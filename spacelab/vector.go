package spacelab

import "github.com/racerxdl/spacelabview/spaceproto"

type Vector3 struct {
	X, Y, Z float64
}

func (v Vector3) ToProto() *spaceproto.Vector3 {
	return &spaceproto.Vector3{X: v.X, Y: v.Y, Z: v.Z}
}
