#!/usr/bin/env python3
"""
Debug script to check coordinate system
"""

import numpy as np
import pygame
from emotion_contagion import EmotionContagion

# Initialize pygame
pygame.init()

# Create a small test game
game = EmotionContagion(width=100, height=100, cell_size=10)

print("Testing coordinate system...")

# Set up a simple pattern
game.grid[5, 5] = True
game.grid[4, 5] = True
game.grid[6, 5] = True

print(f"Grid shape: {game.grid.shape}")
print(f"Grid width: {game.grid_width}, Grid height: {game.grid_height}")

print(f"\nDirect access:")
print(f"  game.grid[4, 5] = {game.grid[4, 5]}")
print(f"  game.grid[5, 5] = {game.grid[5, 5]}")
print(f"  game.grid[6, 5] = {game.grid[6, 5]}")

print(f"\nTesting neighbor access for (5, 6):")
for dy in range(-1, 2):
    for dx in range(-1, 2):
        if dx == 0 and dy == 0:
            continue
        nx, ny = 5 + dx, 6 + dy
        if 0 <= nx < game.grid_width and 0 <= ny < game.grid_height:
            # Check both coordinate orders
            value1 = game.grid[ny, nx]  # Current way
            value2 = game.grid[nx, ny]  # Alternative way
            print(f"  ({nx}, {ny}): grid[ny,nx]={value1}, grid[nx,ny]={value2}")

pygame.quit() 