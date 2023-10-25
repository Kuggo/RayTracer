from typing import Optional
from random import randint
from utils import *
import pygame as pg


# Screen

class Screen:
    def __init__(self, width_pix: int, height_pix: int, pixel_size: int, title: str):
        self.width_pix = width_pix
        self.height_pix = height_pix
        self.pixel_size = pixel_size
        self.title = title
        self.display = pg.display.set_mode((self.width_pix * self.pixel_size, self.height_pix * self.pixel_size))
        pg.display.set_caption(self.title)
        return

    def get_camera(self, config, world: 'World'):
        return Camera(self, world, config.start_camera_pos, config.start_camera_dir, config.start_camera_up_dir, config.fov, config.pixels_per_unit)

    def update(self):
        pg.display.flip()
        return

    def in_bounds(self, x, y):
        return -(self.width_pix >> 1) <= x < (self.width_pix >> 1) and -(self.height_pix >> 1) <= y < (self.height_pix >> 1)

    def center_pixel(self, x, y):
        # up is positive y, right is positive x
        return x + (self.width_pix >> 1), (self.height_pix >> 1) - y

    def draw_pixel(self, x, y, color: Color):
        pg.draw.rect(self.display, color.tuple(), (x * self.pixel_size, y * self.pixel_size, self.pixel_size, self.pixel_size))
        return



# raytracing specific

class Ray:
    def __init__(self, origin: Vector, direction: Vector, divergence = 0):
        self.origin: Vector = origin
        self.direction: Vector = direction.normalize()
        self.divergence: float = divergence
        return

    def trace(self, world: 'World', bounces_left=0) -> Color:
        position = self.origin
        chunk = world.chunk_at(self.origin)
        while position.manhattan_distance(self.origin) < 100:
            if chunk is None:
                return Color(0, 0, 0)
            if len(chunk.voxels) == 0:
                continue

            while chunk == (chunk := world.chunk_at(position)) and position.__floor__() not in chunk.voxels:
                position += self.direction

            if chunk is None or position.__floor__() not in chunk.voxels:
                continue

            voxel = chunk.voxels[position.__floor__()]
            return voxel.color *  (1 - (position.distance(self.origin) / 100))

        return Color(0, 0, 0)


class Voxel:
    def __init__(self, color: Color):
        self.color: Color = color
        transparency = 0
        return


class Chunk:
    def __init__(self, position: Point3D, chunk_size):
        self.position: Point3D = position
        self.chunk_size: int = chunk_size
        self.voxels: dict[Point3D, Voxel] = {}
        return

    def voxel_at(self, position: Point3D) -> Voxel:
        return self.voxels[position.__floor__()]

    def generate_random(self, num_voxels: int):
        for i in range(num_voxels):
            x, y, z = randint(0, self.chunk_size - 1), randint(0, self.chunk_size - 1), randint(0, self.chunk_size - 1)
            self.voxels[Point3D(x, y, z)] = Voxel(Color(255, 255, 255))
        return


class World:
    def __init__(self, chunk_size: int = 16):
        self.chunk_size: int = chunk_size
        self.chunks: dict[Point3D, Chunk] = {}
        return

    def voxel_at(self, position: Point3D) -> Voxel:
        chunk = self.chunk_at(position)
        voxel = chunk.voxel_at(position)
        return voxel

    def chunk_at(self, position: Point3D) -> Optional[Chunk]:
        position = position // self.chunk_size
        if position in self.chunks:
            return self.chunks[position]
        else:
            return None

    def set_voxel_at(self, position: Point3D, voxel: Voxel):
        chunk = self.chunk_at(position)
        chunk.voxels[position % self.chunk_size] = voxel
        return

    def remove_voxel_at(self, position: Point3D):
        chunk = self.chunk_at(position)
        return  chunk.voxels.pop(position % self.chunk_size)

    def create_chunk_at(self, position: Point3D):
        pos = position // self.chunk_size
        c = Chunk(pos, self.chunk_size)
        self.chunks[pos] = c
        return c

    def unload_chunk_at(self, position: Point3D):
        return self.chunks.pop(position)

    def load_chunk(self, chunk: Chunk):
        self.chunks[chunk.position] = chunk
        return

    def unload_chunk(self, chunk: Chunk):
        return self.chunks.pop(chunk.position)



# Camera Orientation

class Camera:
    def __init__(self, screen: 'Screen', world: World, position: Point3D, direction: Point3D, up_vector: Point3D, fov: float, pixels_per_unit: int):
        self.screen = screen
        self.pixels_per_unit: int = pixels_per_unit
        self.width_units: float = self.screen.width_pix / self.pixels_per_unit
        self.height_units: float = self.screen.height_pix / self.pixels_per_unit

        self.world: World = world
        self.position: Point3D = position
        self.front_direction: Vector = direction.normalize()
        self.up_vector: Vector = up_vector.normalize()

        self.fov: float = 0
        self.focal_length: float = 0
        self.set_fov(fov)
        return

    def set_fov(self, fov):
        self.fov = fov
        self.focal_length = (self.width_units / 2) / math.tan(fov / 2)
        return

    def rotate_pov(self, pitch: float = 0, yaw: float = 0, roll: float = 0):
        # yaw
        self.front_direction = self.front_direction.rotate_xz(-yaw)
        self.up_vector = self.up_vector.rotate_xz(-yaw)

        # pitch
        right_vec = self.front_direction.cross(self.up_vector)
        self.front_direction = self.front_direction.rotate_around(right_vec, pitch)
        self.up_vector = self.up_vector.rotate_around(right_vec, pitch)

        # roll
        self.up_vector = self.up_vector.rotate_around(self.front_direction, roll)
        return

    def move(self, direction: Vector):
        self.position += direction
        return

    def world_to_camera(self, p: Vector) -> Vector:
        return p - self.position

    def draw_frame(self) -> None:
        print(self.position)
        up_vec = self.up_vector * (self.height_units / self.screen.height_pix)
        right_vec = self.front_direction.cross(self.up_vector) * (self.width_units / self.screen.width_pix)
        front_vec = self.front_direction * self.focal_length

        """        for x in range(-(self.screen.width_pix >> 1), self.screen.width_pix >> 1):
            for y in range(self.screen.height_pix >> 1, -(self.screen.height_pix >> 1), -1):"""
        for x in range(0, self.screen.width_pix):
            for y in range(0, self.screen.height_pix):
                x_c, y_c = x - (self.screen.width_pix >> 1), (self.screen.height_pix >> 1) - y
                v = front_vec + right_vec * x_c + up_vec * y_c
                ray = Ray(self.position, v)
                color = ray.trace(self.world)
                self.screen.draw_pixel(x, y, color)

        return
