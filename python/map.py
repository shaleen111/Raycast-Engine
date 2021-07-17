import pygame

block_size = 64
map_w = 8
map_h = 8

map = [[1, 1, 1, 1, 1, 1, 1, 1,],
       [1, 0, 1, 0, 0, 0, 0, 1,],
       [1, 0, 1, 0, 0, 0, 0, 1,],
       [1, 0, 1, 0, 0, 0, 0, 1,],
       [1, 0, 0, 0, 1, 1, 0, 1,],
       [1, 0, 0, 0, 1, 1, 0, 1,],
       [1, 0, 0, 0, 0, 0, 0, 1,],
       [1, 1, 1, 1, 1, 1, 1, 1]]


def draw_map(display):
    for y in range(map_h):
        for x in range(map_w):
            col = (255, 255, 255) if map[map_w*y + x] == 1 else (0, 0, 0)

            pygame.draw.rect(display, col, (64 * x + 1, 64 * y + 1,
                             block_size - 2, block_size - 2))


def update_map(e):
    if e.type == pygame.MOUSEBUTTONDOWN:
        x, y = pygame.mouse.get_pos()
        x = x // block_size
        y = y // block_size
        if x < 8 and y < 8:
            map[map_w*y + x] = not map[map_w*y + x]
