import pygame
import map
import player


def main():
    pygame.init()

    pygame.display.set_caption("Raycast Engine")

    screen = pygame.display.set_mode((1024, 512))

    keys = {"UP": False, "RIGHT": False, "DOWN": False, "LEFT": False}

    quit = False

    p = player.Player()

    while not quit:

        for event in pygame.event.get():

            if event.type == pygame.QUIT:
                quit = True

            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    keys["UP"] = True
                elif event.key == pygame.K_DOWN:
                    keys["DOWN"] = True
                elif event.key == pygame.K_RIGHT:
                    keys["RIGHT"] = True
                elif event.key == pygame.K_LEFT:
                    keys["LEFT"] = True

            elif event.type == pygame.KEYUP:
                if event.key == pygame.K_UP:
                    keys["UP"] = False
                elif event.key == pygame.K_DOWN:
                    keys["DOWN"] = False
                elif event.key == pygame.K_RIGHT:
                    keys["RIGHT"] = False
                elif event.key == pygame.K_LEFT:
                    keys["LEFT"] = False

            else:
                map.change_map(event)

        # Update player x & y
        player.move_player(keys)

        # Draw screen & player
        screen.fill((150, 150, 180))
        map.draw_map(screen)
        player.draw_player(screen)
        pygame.display.flip()

def raycast():
    pass

if __name__ == "__main__":
    main()
