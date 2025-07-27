#!/usr/bin/env python3
"""
Debug script to see what's happening with neighbor detection
"""

import numpy as np
import pygame
from emotion_contagion import EmotionContagion

# Initialize pygame
pygame.init()

# Create a small test game
game = EmotionContagion(width=100, height=100, cell_size=10)

# Set up the same pattern as the test
print("Setting up test pattern...")
game.grid[5, 5] = True
game.emotions[5, 5] = 'joy'
game.intensities[5, 5] = 0.8

game.grid[4, 5] = True
game.emotions[4, 5] = 'sadness'
game.intensities[4, 5] = 0.6

game.grid[6, 5] = True
game.emotions[6, 5] = 'anger'
game.intensities[6, 5] = 0.4

print(f"Grid state after setup:")
print(f"  (5,5): alive={game.grid[5, 5]}, emotion={game.emotions[5, 5]}, intensity={game.intensities[5, 5]}")
print(f"  (4,5): alive={game.grid[4, 5]}, emotion={game.emotions[4, 5]}, intensity={game.intensities[4, 5]}")
print(f"  (6,5): alive={game.grid[6, 5]}, emotion={game.emotions[6, 5]}, intensity={game.intensities[6, 5]}")

# Test neighbor detection for cell (5, 6)
print(f"\nTesting neighbor detection for cell (5, 6)...")
neighbor_emotions = game.get_neighbor_emotions(5, 6)

print(f"Found {len(neighbor_emotions)} neighbor emotions:")
for emotion, intensity in neighbor_emotions:
    print(f"  {emotion}: {intensity}")

# Check what neighbors should be found for (5, 6) - using correct coordinate system
print(f"\nNeighbors for (5, 6) - checking each neighbor:")
for dy in range(-1, 2):
    for dx in range(-1, 2):
        if dx == 0 and dy == 0:
            continue
        nx, ny = 5 + dx, 6 + dy
        if 0 <= nx < game.grid_width and 0 <= ny < game.grid_height:
            alive = game.grid[ny, nx]  # Use correct (y, x) order
            emotion = game.emotions[ny, nx]
            intensity = game.intensities[ny, nx]
            meets_criteria = (alive and emotion is not None and intensity > 0.1)
            print(f"  ({nx}, {ny}): grid[{ny},{nx}]={alive}, emotion={emotion}, intensity={intensity}, meets_criteria={meets_criteria}")

# Let's also check the actual grid values to make sure they're correct
print(f"\nActual grid values:")
print(f"  game.grid[4, 5] = {game.grid[4, 5]}")
print(f"  game.grid[5, 5] = {game.grid[5, 5]}")
print(f"  game.grid[6, 5] = {game.grid[6, 5]}")

# Let's check what the function is actually doing step by step
print(f"\nStep-by-step neighbor check for (5, 6):")
for dy in range(-1, 2):
    for dx in range(-1, 2):
        if dx == 0 and dy == 0:
            continue
        nx, ny = 5 + dx, 6 + dy
        print(f"  dx={dx}, dy={dy} -> nx={nx}, ny={ny}")
        if 0 <= nx < game.grid_width and 0 <= ny < game.grid_height:
            print(f"    -> grid[{ny}, {nx}] = {game.grid[ny, nx]}")
        else:
            print(f"    -> out of bounds")

pygame.quit() 