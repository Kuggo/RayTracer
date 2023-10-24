import math
from typing import Optional

import pygame as pg


fp_tolerance = 1e-6
rgb_bound = 255
meter_size = 1
chunk_size = 16


# auxiliary functions

def fp_equals(a, b) -> bool:
    return abs(a - b) < fp_tolerance

def lerp(a, b, t):
    return a + (b - a) * t


def inv_lerp(a, b, v):
    return (v - a) / (b - a)


def clamp(a, b, v):
    return min(b, max(a, v))



# 2D Screen objects

class Color:
    def __init__(self, r, g, b, a=255):
        self.r = r
        self.g = g
        self.b = b
        self.a = a
        return

    @staticmethod
    def from_tuple(t: tuple[int, int, int, int]):
        return Color(*t)

    @staticmethod
    def from_hex(hex_str: str):
        if len(hex_str) == 7:
            return Color(int(hex_str[1:3], 16), int(hex_str[3:5], 16), int(hex_str[5:7], 16))
        else:
            return Color(int(hex_str[1:3], 16), int(hex_str[3:5], 16), int(hex_str[5:7], 16), int(hex_str[7:9], 16))

    def tuple(self):
        return clamp(0, rgb_bound, round(self.r)), clamp(0, rgb_bound, round(self.g)), clamp(0, rgb_bound, round(self.b))

    def round(self):
        return Color(clamp(0, rgb_bound, round(self.r)),
                     clamp(0, rgb_bound, round(self.g)),
                     clamp(0, rgb_bound, round(self.b)))

    def __add__(self, other: 'Color'):
        return Color(self.r + other.r, self.g + other.g, self.b + other.b)

    def __sub__(self, other: 'Color'):
        return Color(self.r - other.r, self.g - other.g, self.b - other.b)

    def __mul__(self, other: int|float):
        return Color(self.r * other, self.g * other, self.b * other)

    def __truediv__(self, other: int|float):
        return Color(self.r / other, self.g / other, self.b / other)

    def __eq__(self, other: 'Color'):
        return self.r == other.r and self.g == other.g and self.b == other.b

    def __ne__(self, other: 'Color'):
        return not self == other

    def __round__(self, n=None):
        return Color(round(self.r, n), round(self.g, n), round(self.b, n))

    def __hash__(self):
        return hash((self.r, self.g, self.b))

    def __repr__(self):
        return f"#{int(self.r):0{2}X}{int(self.g):0{2}X}{int(self.b):0{2}X}"


class Pixel:
    def __init__(self, x: int | float, y: int | float, color: 'Color' = Color(255, 255, 255)):
        self.x: int = round(x)
        self.y: int = round(y)
        self.color: Color = color
        return

    def __eq__(self, other: 'Pixel'):
        return self.x == other.x and self.y == other.y

    def __ne__(self, other: 'Pixel'):
        return not self == other

    def __lt__(self, other: 'Pixel'):
        return self.y < other.y or (self.y == other.y and self.x < other.x)

    def __le__(self, other: 'Pixel'):
        return self < other or self == other

    def __gt__(self, other: 'Pixel'):
        return not self <= other

    def __ge__(self, other: 'Pixel'):
        return not self < other

    def __hash__(self):
        return hash((self.x, self.y))

    def __repr__(self):
        return f"({self.x}, {self.y})"

    @staticmethod
    def from_tuple(t: tuple[int | float, ...]):
        if len(t) == 2:
            return Pixel(t[0], t[1], color=Color(255, 255, 255))
        elif len(t) >= 3:
            return Vector(t[0], t[1], t[2])
        else:
            assert False

    def distance(self, other: 'Pixel') -> int | float:
        return ((self.x - other.x) ** 2 + (self.y - other.y) ** 2) ** 0.5

    def manhattan_distance(self, other: 'Pixel') -> int | float:
        return abs(self.x - other.x) + abs(self.y - other.y)



# 3D space

class Vector:
    def __init__(self, x: int | float, y: int | float, z: int | float):
        self.x = x
        self.y = y
        self.z: int|float = z
        return

    def __repr__(self):
        return f"({self.x:.4f}, {self.y:.4f}, {self.z:.4f})"

    def __hash__(self):
        return hash((self.x, self.y, self.z))

    def __eq__(self, other: 'Vector'):
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __ne__(self, other: 'Vector'):
        return not self == other

    def __add__(self, other: 'Vector'):
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other: 'Vector'):
        return Vector(self.x - other.x, self.y - other.y, self.z - other.z)

    def __neg__(self):
        return Vector(-self.x, -self.y, -self.z)

    def __mul__(self, other: int|float):
        return Vector(self.x * other, self.y * other, self.z * other)

    def __round__(self, n=None):
        return Vector(round(self.x, n), round(self.y, n), round(self.z, n))

    def fp_equals(self, other: 'Vector'):
        return abs(self.x - other.x) < fp_tolerance and abs(self.y - other.y) < fp_tolerance and \
            abs(self.z - other.z) < fp_tolerance

    def distance(self, other: 'Vector') -> int | float:
        return ((self.x - other.x) ** 2 + (self.y - other.y) ** 2 + (self.z - other.z) ** 2) ** 0.5

    def manhattan_distance(self, other: 'Vector') -> int | float:
        return abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)

    def rotate_pitch(self, pitch: float) -> 'Vector':
        cos_pitch = math.cos(pitch)
        sin_pitch = math.sin(pitch)
        y = self.y * cos_pitch - self.z * sin_pitch
        z = self.y * sin_pitch + self.z * cos_pitch
        return Vector(self.x, y, z)

    def rotate_yaw(self, yaw: float) -> 'Vector':
        cos_yaw = math.cos(yaw)
        sin_yaw = math.sin(yaw)
        x = self.x * cos_yaw - self.z * sin_yaw
        z = self.x * sin_yaw + self.z * cos_yaw
        return Vector(x, self.y, z)

    def rotate_roll(self, roll: float) -> 'Vector':
        cos_roll = math.cos(roll)
        sin_roll = math.sin(roll)
        x = self.x * cos_roll - self.y * sin_roll
        y = self.x * sin_roll + self.y * cos_roll
        return Vector(x, y, self.z)

    def get_polar(self) -> tuple[int|float, int|float]:
        """Returns the polar coordinates of the vector, pitch and yaw"""
        m = self.magnitude()
        if m == 0:
            return 0, 0
        pitch = math.asin(self.y / self.magnitude())
        yaw = math.atan2(self.x, self.z)
        return pitch, yaw

    def rotate_to_plane(self, point: 'Vector') -> 'Vector':
        """Returns the vector projected on the other vector"""
        pitch, yaw = self.get_polar()
        return point.rotate_pitch(-pitch).rotate_yaw(-yaw)

    def dot(self, other: 'Vector') -> int | float:
        return self.x * other.x + self.y * other.y + self.z * other.z

    def cross(self, other: 'Vector') -> 'Vector':
        return Vector(self.y * other.z - self.z * other.y,
                      self.z * other.x - self.x * other.z,
                      self.x * other.y - self.y * other.x)

    def colinear(self, other: 'Vector') -> bool:
        return fp_equals(self.dot(other), 0)

    def normalize(self) -> 'Vector':
        return self * (1 / self.magnitude())

    def magnitude(self) -> int|float:
        return math.sqrt(self.x ** 2 + self.y ** 2 + self.z ** 2)

    def angle(self, other: 'Vector') -> int|float:
        return math.acos(self.dot(other) / (self.magnitude() * other.magnitude()))


Point3D = Vector


class Line3D:
    def __init__(self, a: Point3D, b: Point3D):
        if a < b:
            self.a: Point3D = a
            self.b: Point3D = b
        else:
            self.a: Point3D = b
            self.b: Point3D = a
        return

    def __repr__(self):
        return f"({self.a}, {self.b})"

    def __eq__(self, other: 'Line3D'):
        return (self.a == other.a and self.b == other.b) or (self.a == other.b and self.b == other.a)

    def __hash__(self):
        return hash((self.a, self.b))

    def length(self) -> float:
        return self.a.distance(self.b)

    def manhattan_length(self) -> float:
        return self.a.manhattan_distance(self.b)

