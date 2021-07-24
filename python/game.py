import pygame
from pygame.math import Vector2

import math

import map
from player import Player

def main():
    pygame.init()

    pygame.display.set_caption("Raycast Engine")
    width, height = 1024, 512
    screen = pygame.display.set_mode((width, height))

    key_held = {"w": False, "a": False, "s": False, "d": False}

    quit = False

    p = Player(1.5, 1.5, 0, -1)
    plane = Vector2(0.66, 0)

    last_frame = pygame.time.get_ticks()

    angular_acc = 3.0
    acc = 2.5
    while not quit:
        # Update player x & y
        # p.move(keys)

        # Draw screen & player
        screen.fill((150, 150, 180))
        current_frame = pygame.time.get_ticks()
        dt = (current_frame - last_frame) / 1000
        last_frame = current_frame

        print(1/dt)

        for x in range(width):
            camera_x = 2 * x / width - 1
            ray_dir = p.dir + camera_x * plane
            map_pos = Vector2(int(p.pos.x), int(p.pos.y))

            if ray_dir.x == 0:
                delta_x = 1
                delta_y = 0
            elif ray_dir.y == 0:
                delta_x = 0
                delta_y = 1
            else:
                delta_x = abs(1 / ray_dir.x)
                delta_y = abs(1 / ray_dir.y)

            step = Vector2()
            if ray_dir.x < 0:
                step.x = -1
                side_x = (p.pos.x - map_pos.x) * delta_x
            else:
                step.x = 1
                side_x = (map_pos.x + 1 - p.pos.x) * delta_x

            if ray_dir.y < 0:
                step.y = -1
                side_y = (p.pos.y - map_pos.y) * delta_y
            else:
                step.y = 1
                side_y = (map_pos.y + 1 - p.pos.y) * delta_y

            hit = False
            side = None
            while not hit:
                if side_x < side_y:
                    side_x += delta_x
                    map_pos.x += step.x
                    side = 0
                else:
                    side_y += delta_y
                    map_pos.y += step.y
                    side = 1

                if map.map[int(map_pos.y)][int(map_pos.x)] == 1:
                    hit = True

            if side is None:
                continue

            perp_dist_wall = None
            if side == 0:
                correction = (1 - step.x) / 2
                perp_dist_wall = (map_pos.x - p.pos.x + correction) / ray_dir.x
            else:
                correction = (1 - step.y) / 2
                perp_dist_wall = (map_pos.y - p.pos.y + correction) / ray_dir.y

            line_height = int(height / perp_dist_wall)
            line_start = (height - line_height) / 2
            line_end = (height + line_height) / 2

            color = (255, 255, 255)
            if side == 1:
                color = (128, 128, 128)

            pygame.draw.line(screen, color, (x, line_start), (x, line_end))
        vector_for_first_col = -plane
        pygame.draw.line(screen, (0, 255, 0), (width/2, height/2), (width/2 + vector_for_first_col.x * 100, height/2 + vector_for_first_col.y * 100))
        pygame.draw.line(screen, (0, 0, 255), (width/2 + vector_for_first_col.x * 100, height/2 + vector_for_first_col.y * 100), (width/2 + vector_for_first_col.x * 105, height/2 + vector_for_first_col.y * 105))


        pygame.display.flip()

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                quit = True

            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_w:
                    key_held["w"] = True

                elif event.key == pygame.K_s:
                    key_held["s"] = True

                elif event.key == pygame.K_d:
                    key_held["d"] = True

                elif event.key == pygame.K_a:
                    key_held["a"] = True

            elif event.type == pygame.KEYUP:
                if event.key == pygame.K_w:
                    key_held["w"] = False

                elif event.key == pygame.K_s:
                    key_held["s"] = False

                elif event.key == pygame.K_d:
                    key_held["d"] = False

                elif event.key == pygame.K_a:
                    key_held["a"] = False

        if key_held["w"]:
            if map.map[int(p.pos.y)][int(p.pos.x + p.dir.x * acc * dt)] == 0:
                p.pos.x += p.dir.x * acc * dt
            if map.map[int(p.pos.y + p.dir.y * acc * dt)][int(p.pos.x)] == 0:
                p.pos.y += p.dir.y * acc * dt
        elif key_held["s"]:
            if map.map[int(p.pos.y)][int(p.pos.x - p.dir.x * acc * dt)] == 0:
                p.pos.x -= p.dir.x * acc * dt
            if map.map[int(p.pos.y - p.dir.y * acc * dt)][int(p.pos.x)] == 0:
                p.pos.y -= p.dir.y * acc * dt

        if key_held["d"]:
            old_dir = p.dir
            p.dir = Vector2(old_dir.x * math.cos(-angular_acc * dt) + old_dir.y * math.sin(-angular_acc * dt),
                            - old_dir.x * math.sin(-angular_acc * dt) + old_dir.y * math.cos(-angular_acc * dt))
            old_plane = plane
            plane = Vector2(old_plane.x * math.cos(-angular_acc * dt) + old_plane.y * math.sin(-angular_acc * dt),
                            - old_plane.x * math.sin(-angular_acc * dt) + old_plane.y * math.cos(-angular_acc * dt))
            print(plane, p.dir)
        elif key_held["a"]:
            old_dir = p.dir
            p.dir = Vector2(old_dir.x * math.cos(angular_acc * dt) + old_dir.y * math.sin(angular_acc * dt),
                            - old_dir.x * math.sin(angular_acc * dt) + old_dir.y * math.cos(angular_acc * dt))
            old_plane = plane
            plane = Vector2(old_plane.x * math.cos(angular_acc * dt) + old_plane.y * math.sin(angular_acc * dt),
                            - old_plane.x * math.sin(angular_acc * dt) + old_plane.y * math.cos(angular_acc * dt))

if __name__ == "__main__":
    main()
