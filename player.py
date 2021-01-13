import pygame
import math

p_x, p_y, p_w, p_h = 300, 200, 10, 10
speed = 0.2

ray_l = 25
angle = 0


def draw_player(display):
    pygame.draw.rect(display, (255, 0, 0),
                     (p_x, p_y, p_w, p_h))
    pygame.draw.line(display, (255, 0, 255), (p_x + p_w/2, p_y + p_h/2),
                     (p_x + p_w/2 + math.cos(angle) * ray_l,
                      p_y + p_h/2 + math.sin(angle) * ray_l), 2)


def move_player(keys):
    global p_x
    global p_y
    global angle

    if keys["UP"]:
        p_y += math.sin(angle)*speed
        p_x += math.cos(angle)*speed
    elif keys["DOWN"]:
        p_y -= math.sin(angle)*speed
        p_x -= math.cos(angle)*speed

    if keys["RIGHT"]:
        angle += 0.01
    elif keys["LEFT"]:
        angle -= 0.01
