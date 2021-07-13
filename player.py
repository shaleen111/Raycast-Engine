import pygame
import math

class Player(object):

    def __init__(self, x, y, w, h, v):
        self.x = x
        self.y = y
        self.w = w
        self.h = h

        self.v = v
        self.angle = 0
        self.ray_l = 25

    def draw(self, display):
        pygame.draw.rect(display, (255, 0, 0),
                        (self.x, self.y, self.w, self.h))
        pygame.draw.line(display, (255, 0, 255), (self.x + self.w/2, self.y + self.h/2),
                        (self.x + self.w/2 + math.cos(self.angle) * self.ray_l,
                         self.y + self.h/2 + math.sin(self.angle) * self.ray_l), 2)

    def move(self, keys):
        if keys["UP"]:
            self.y += math.sin(self.angle) * self.speed
            self.x += math.cos(self.angle) * self.speed
        elif keys["DOWN"]:
            self.y -= math.sin(self.angle) * self.speed
            self.x -= math.cos(self.angle) * self.speed

        if keys["RIGHT"]:
            self.angle += 0.01
        elif keys["LEFT"]:
            self.angle -= 0.01
