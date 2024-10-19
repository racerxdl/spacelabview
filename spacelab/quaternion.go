package spacelab

import "github.com/racerxdl/spacelabview/spaceproto"

type Quaternion struct {
	X, Y, Z, W float64
}

func (q Quaternion) ToProto() *spaceproto.Quaternion {
	return &spaceproto.Quaternion{
		X: q.X,
		Y: q.Y,
		Z: q.Z,
		W: q.W,
	}
}
