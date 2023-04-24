package spacelab

type GlobalInfo struct {
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
