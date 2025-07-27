#!/usr/bin/env python3
"""
Simple test to verify grid access
"""

import numpy as np
import pygame
from emotion_contagion import EmotionContagion

# Initialize pygame
pygame.init()

# Create a small test game
game = EmotionContagion(width=100, height=100, cell_size=10)

print("Testing grid access...")

# Set up the grid
game.grid[5, 5] = True
game.grid[4, 5] = True
game.grid[6, 5] = True

print(f"After setup:")
print(f"  game.grid[4, 5] = {game.grid[4, 5]}")
print(f"  game.grid[5, 5] = {game.grid[5, 5]}")
print(f"  game.grid[6, 5] = {game.grid[6, 5]}")

# Check if the grid is being modified somewhere
print(f"\nGrid shape: {game.grid.shape}")
print(f"Grid type: {type(game.grid)}")

# Let's check if there's any automatic clearing happening
print(f"\nChecking if grid is being cleared...")
for i in range(5):
    print(f"  Iteration {i}: grid[4,5]={game.grid[4, 5]}, grid[5,5]={game.grid[5, 5]}, grid[6,5]={game.grid[6, 5]}")

pygame.quit() 