from pygame.math import Vector2
import math

class Player(object):

    # (x, y) -> Position Vector
    # (dx, dy) -> Direction Vector
    def __init__(self, x, y, dx, dy):
        self.pos = Vector2(x, y)
        self.dir = Vector2(dx, dy)
