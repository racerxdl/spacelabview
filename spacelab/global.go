package spacelab

import "github.com/racerxdl/spacelabview/spaceproto"

type GlobalInfo struct {
	SunNormalizedPosition    Vector3
	SunNormalizedX           float64
	SunNormalizedY           float64
	SunNormalizedZ           float64
	SunIntensity             float64
	SmallShipMaxSpeed        float64
	SmallShipMaxAngularSpeed float64
	LargeShipMaxSpeed        float64
	LargeShipMaxAngularSpeed float64
}

func (g1 GlobalInfo) ChangedSince(g2 GlobalInfo) bool {
	return g1.SunIntensity != g2.SunIntensity ||
		g1.SunNormalizedX != g2.SunNormalizedX ||
		g1.SunNormalizedY != g2.SunNormalizedY ||
		g1.SunNormalizedZ != g2.SunNormalizedZ
}

func (g1 GlobalInfo) ToProto() *spaceproto.GlobalInfo {
	return &spaceproto.GlobalInfo{
		SunNormalized:            g1.SunNormalizedPosition.ToProto(),
		SunIntensity:             g1.SunIntensity,
		SmallShipMaxSpeed:        g1.SmallShipMaxSpeed,
		SmallShipMaxAngularSpeed: g1.SmallShipMaxAngularSpeed,
		LargeShipMaxSpeed:        g1.LargeShipMaxSpeed,
		LargeShipMaxAngularSpeed: g1.LargeShipMaxAngularSpeed,
	}
}
