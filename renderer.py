from utils import *


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

    def get_camera(self, config):
        return Camera(self, config.start_camera_pos, config.start_camera_dir, config.start_camera_up_dir, config.fov, config.pixels_per_unit)

    def update(self):
        pg.display.flip()
        return

    def in_bounds(self, x, y):
        return -(self.width_pix >> 1) <= x < (self.width_pix >> 1) and -(self.height_pix >> 1) <= y < (self.height_pix >> 1)

    def center_pixel(self, x, y):
        # up is positive y, right is positive x
        return x + (self.width_pix >> 1), (self.height_pix >> 1) - y

    def draw_pixel(self, x, y, color: Color):
        if self.in_bounds(x, y):
            pg.draw.rect(self.display, color.tuple(), (x * self.pixel_size, y * self.pixel_size, self.pixel_size, self.pixel_size))
        return



# raytracing specific

class Ray:
    def __init__(self, origin: Vector, direction: Vector, divergence = 0, color: Color = Color(255, 255, 255)):
        self.origin: Vector = origin
        self.direction: Vector = direction.normalize()
        self.divergence: float = divergence
        self.color: Color = color
        return


class Voxel:
    def __init__(self, position: Point3D, color: Color = Color(255, 255, 255, 255)):
        self.position: Point3D = position
        self.color: Color = color
        return


class Chunk:
    def __init__(self, position: Point3D):
        self.position: Point3D = position
        self.voxels: list[Optional[Voxel]] = [None] * (chunk_size ** 3)
        return

    def voxel_index(self, position: Point3D) -> int:
        pos = position - self.position
        return pos.x + pos.y * chunk_size + pos.z * chunk_size*chunk_size

    def voxel_at(self, position: Point3D) -> Voxel:
        index = self.voxel_index(position)
        return self.voxels[index]


class World:
    def __init__(self):
        self.chunks: dict[tuple[int, int, int], Chunk] = {}
        return

    @staticmethod
    def chunk_key(position: Point3D) -> tuple[int, int, int]:
        return position.x - (position.x % chunk_size), position.y - (position.y % chunk_size), position.z - (position.z % chunk_size)

    def voxel_at(self, position: Point3D) -> Voxel:
        chunk = self.chunk_at(position)
        voxel = chunk.voxel_at(position)
        return voxel

    def chunk_at(self, position: Point3D) -> Chunk:
        k = self.chunk_key(position)
        if k in self.chunks:
            return self.chunks[k]
        else:
            return Chunk(position)



# Camera Orientation

class Camera:
    def __init__(self, screen: 'Screen', position: Point3D, direction: Point3D, up_vector: Point3D, fov: float, pixels_per_unit: int):
        self.screen = screen
        self.pixels_per_unit: int = pixels_per_unit
        self.width_units: float = self.screen.width_pix / self.pixels_per_unit
        self.height_units: float = self.screen.height_pix / self.pixels_per_unit

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
        self.front_direction = self.front_direction.rotate_pitch(pitch)
        self.front_direction = self.front_direction.rotate_yaw(yaw)
        self.front_direction = self.front_direction.rotate_roll(roll)
        self.up_vector = self.up_vector.rotate_pitch(pitch)
        self.up_vector = self.up_vector.rotate_yaw(yaw)
        self.up_vector = self.up_vector.rotate_roll(roll)
        return

    def move(self, direction: Vector):
        self.position += direction
        return

    def world_to_camera(self, p: Vector) -> Vector:
        return p - self.position

    def draw_frame(self, world) -> None:
        up_vec = self.up_vector * (self.height_units / self.screen.height_pix)
        right_vec = self.front_direction.cross(self.up_vector) * (self.width_units / self.screen.width_pix)

        for x in range(self.screen.width_pix):
            for y in range(self.screen.height_pix):
                self.screen.draw_pixel(x, y, Color(0, 0, 0))
        return
