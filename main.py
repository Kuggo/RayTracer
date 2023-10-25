from renderer import *
import pygame as pg


class Settings:
    def __init__(self, screen_width_pix: int = 100, screen_height_pix: int = 100, pixel_size: int = 8, fps: int = 10,
                 fov: float = math.pi / 2, pixels_per_unit: int = 555, start_camera_pos: Point3D = Point3D(0, 0, 0),
                 start_camera_dir: Vector = Vector(0, 0, 1), start_camera_up_dir = Vector(0, 1, 0),
                 mouse_sensitivity: float = 0.1, scroll_sensitivity: float = 0.1, zoom_sensitivity: float = 0.1):

        self.screen_width_pix: int = screen_width_pix
        self.screen_height_pix: int = screen_height_pix
        self.pixel_size: int = pixel_size
        self.pixels_per_unit: int = pixels_per_unit
        self.fps: int = fps
        self.fov: float = fov

        self.start_camera_pos: Point3D = start_camera_pos
        self.start_camera_dir: Vector = start_camera_dir
        self.start_camera_up_dir: Vector = start_camera_up_dir

        self.mouse_sensitivity: float = mouse_sensitivity
        self.scroll_sensitivity: float = scroll_sensitivity
        self.zoom_sensitivity: float = zoom_sensitivity
        return


def generate_world():
    world = World()
    c0 = world.create_chunk_at(Point3D(0, 0, 0))
    c0.voxels[Point3D(8, 8, 8)] = Voxel(Color(255, 255, 255))
    world.load_chunk(c0)
    return world


def motion(config: Settings, camera: Camera, dt: float):
    screen_center = [(config.screen_width_pix * config.pixel_size) >> 1, (config.screen_height_pix * config.pixel_size) >> 1]
    ctrl_pressed = False
    pitch = 0
    yaw = 0
    roll = 0
    movement = Vector(0, 0, 0)

    keys = pg.key.get_pressed()
    if keys[pg.K_LCTRL] or keys[pg.K_RCTRL]:
        ctrl_pressed = True
    if keys[pg.K_w]:
        movement.z += 1
    if keys[pg.K_a]:
        movement.x -= 1
    if keys[pg.K_s]:
        movement.z -= 1
    if keys[pg.K_d]:
        movement.x += 1

    for e in pg.event.get():
        if e.type == pg.QUIT or (e.type == pg.KEYDOWN and e.key == pg.K_ESCAPE):
            pg.quit()
            return True

        elif e.type == pg.MOUSEMOTION:
            relative_pos = pg.mouse.get_rel()
            pitch += (-relative_pos[1] / config.pixels_per_unit) * config.mouse_sensitivity
            yaw += (-relative_pos[0] / config.pixels_per_unit) * config.mouse_sensitivity
            pg.mouse.set_pos(screen_center)
            buttons = pg.mouse.get_pressed()
            if buttons[0]:  # TODO: Add mouse button functionality
                pass
            elif buttons[1]:
                pass
            elif buttons[2]:
                pass

        elif e.type == pg.MOUSEWHEEL:
            if ctrl_pressed:
                camera.set_fov(camera.fov + e.y * config.zoom_sensitivity)
            else:
                roll = (-e.y if e.flipped else e.y) * config.scroll_sensitivity

    camera.rotate_pov(pitch, yaw, roll)
    # movement = movement.rotate_yaw(camera.yaw)
    reference = Vector(camera.front_direction.x, 0, camera.front_direction.z)
    movement = movement.rotate_to_plane(reference)

    # up down doesn't depend on camera rotation
    if keys[pg.K_SPACE]:
        movement.y += 1
    if keys[pg.K_LSHIFT] or keys[pg.K_RSHIFT]:
        movement.y -= 1

    movement *= dt
    camera.move(movement)

    return False


def main_loop(config: Settings, screen: Screen):
    world = generate_world()
    camera = screen.get_camera(config, world)

    dt = 1 / config.fps
    clock = pg.time.Clock()
    while True:
        if motion(config, camera, dt):
            break

        camera.draw_frame()

        screen.update()
        dt = clock.tick(config.fps) / 1000
        print(f'fps = {round(1 / dt, 2)}')
    return


def main():
    screen_width_pix = 100  # in pixels
    screen_height_pix = 100  # in pixels
    pixel_size = 8
    fps = 10
    fov = math.pi / 2
    pixels_per_unit = 555
    start_camera_pos = Point3D(0, 0, 0)
    start_camera_dir = Vector(0, 0, 1)
    start_camera_up_dir = Vector(0, 1, 0)
    mouse_sensitivity = 2
    scroll_sensitivity = 0.1
    zoom_sensitivity = 0.1

    config = Settings(screen_width_pix, screen_height_pix, pixel_size, fps, fov, pixels_per_unit, start_camera_pos,
                      start_camera_dir, start_camera_up_dir, mouse_sensitivity=mouse_sensitivity,
                      scroll_sensitivity=scroll_sensitivity, zoom_sensitivity=zoom_sensitivity)

    # check to see if screen gets too big
    if config.screen_width_pix * pixel_size > 1920 or config.screen_height_pix * pixel_size > 1080:
        exit('Screen size too big')

    screen = Screen(config.screen_width_pix, config.screen_height_pix, config.pixel_size, '3D Renderer')
    pg.mouse.set_visible(False)

    main_loop(config, screen)

    return None


if __name__ == '__main__':
    main()