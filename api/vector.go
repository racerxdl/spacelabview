package api

type Vec struct {
	X float64
}

type Vec2D struct {
	Vec
	Y float64
}

type Vec3D struct {
	Vec2D
	Z float64
}
