import math
import vectormath as vmath

def pixel_to_latitude(face, x_pixel, y_pixel, face_texture_width, face_texture_height):
    u = (x_pixel + 0.5) / face_texture_width * 2.0 - 1.0
    v = (y_pixel + 0.5) / face_texture_height * 2.0 - 1.0

    if face == "up":
        point = vmath.Vector3(u, 1.0, -v)
    elif face == "down":
        point = vmath.Vector3(u, -1.0, v)
    elif face == "left":
        point = vmath.Vector3(-1.0, v, -u)
    elif face == "right":
        point = vmath.Vector3(1.0, v, u)
    elif face == "back":
        point = vmath.Vector3(-u, v, 1.0)
    elif face == "front":
        point = vmath.Vector3(u, v, -1.0)

    point_on_sphere = point.normalize()
    latitude = math.asin(point_on_sphere.y)
    latitude_degrees = math.degrees(latitude)

    return latitude_degrees

def compute_lut(x, y, width, height, p):
    return x, y, pixel_to_latitude(p, x, y, width, height)

rad2deg = 360.0 / (math.pi * 2)

cubemap = ["front", "back", "down", "up", "left", "right"]