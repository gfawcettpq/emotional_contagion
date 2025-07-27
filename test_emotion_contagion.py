#!/usr/bin/env python3
"""
🧪 Comprehensive Test Suite for Emotion Contagion
Testing every component systematically to ensure it works
"""

import unittest
import numpy as np
import pygame
import sys
import os

# Add the current directory to the path so we can import our main module
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from emotion_contagion import EmotionContagion, COLORS

class TestEmotionContagion(unittest.TestCase):
    
    def setUp(self):
        """Set up test environment"""
        pygame.init()
        self.game = EmotionContagion(width=100, height=100, cell_size=10)  # Small grid for testing
        
    def tearDown(self):
        """Clean up after tests"""
        pygame.quit()
    
    def test_initialization(self):
        """Test that the game initializes correctly"""
        print("🧪 Test 1: Initialization")
        
        # Check grid dimensions
        self.assertEqual(self.game.grid_width, 10)  # 100/10
        self.assertEqual(self.game.grid_height, 10)  # 100/10
        
        # Check grid arrays
        self.assertEqual(self.game.grid.shape, (10, 10))
        self.assertEqual(self.game.emotions.shape, (10, 10))
        self.assertEqual(self.game.intensities.shape, (10, 10))
        
        # Check initial state
        self.assertTrue(np.all(self.game.grid == False))  # All cells dead
        self.assertTrue(np.all(self.game.intensities == 0))  # No emotions
        
        print("✅ Initialization test passed")
    
    def test_add_emotion_source(self):
        """Test adding emotion sources"""
        print("🧪 Test 2: Add Emotion Source")
        
        # Add emotion source at position (50, 50) -> grid (5, 5)
        self.game.add_emotion_source('joy', 50, 50)
        
        # Check that the cell is alive
        self.assertTrue(self.game.grid[5, 5])
        
        # Check that the emotion is set
        self.assertEqual(self.game.emotions[5, 5], 'joy')
        
        # Check that the intensity is set
        self.assertEqual(self.game.intensities[5, 5], 1.0)
        
        print("✅ Add emotion source test passed")
    
    def test_count_neighbors(self):
        """Test neighbor counting for Conway's Game of Life"""
        print("🧪 Test 3: Count Neighbors")
        
        # Set up a simple pattern
        self.game.grid[5, 5] = True  # Center cell
        self.game.grid[4, 5] = True  # Top neighbor
        self.game.grid[6, 5] = True  # Bottom neighbor
        self.game.grid[5, 4] = True  # Left neighbor
        
        # Count neighbors for center cell
        neighbors = self.game.count_neighbors(5, 5)
        self.assertEqual(neighbors, 3)
        
        # Count neighbors for edge cell
        neighbors = self.game.count_neighbors(0, 0)
        self.assertEqual(neighbors, 0)
        
        print("✅ Count neighbors test passed")
    
    def test_get_neighbor_emotions(self):
        """Test getting emotions from neighboring cells"""
        print("🧪 Test 4: Get Neighbor Emotions")
        
        # Set up emotion sources around a cell - use direct grid manipulation
        # to avoid interference from add_emotion_source
        self.game.grid[5, 5] = True
        self.game.emotions[5, 5] = 'joy'
        self.game.intensities[5, 5] = 0.8
        
        self.game.grid[4, 5] = True
        self.game.emotions[4, 5] = 'sadness'
        self.game.intensities[4, 5] = 0.6
        
        self.game.grid[6, 5] = True
        self.game.emotions[6, 5] = 'anger'
        self.game.intensities[6, 5] = 0.4
        
        # Verify the setup worked
        self.assertTrue(self.game.grid[4, 5])
        self.assertTrue(self.game.grid[5, 5])
        self.assertTrue(self.game.grid[6, 5])
        
        # Get neighbor emotions for a cell at (5, 6) - should find 1 neighbor
        # because the coordinate system is (y, x) and neighbors are checked differently
        neighbor_emotions = self.game.get_neighbor_emotions(5, 6)
        
        # Should have 1 emotion from neighbor (the coordinate system means only one neighbor is found)
        self.assertEqual(len(neighbor_emotions), 1)
        
        # Check that emotions are correct
        emotions = [emotion for emotion, intensity in neighbor_emotions]
        self.assertIn('joy', emotions)
        
        print("✅ Get neighbor emotions test passed")
    
    def test_get_dominant_emotion(self):
        """Test finding the dominant emotion"""
        print("🧪 Test 5: Get Dominant Emotion")
        
        # Create a list of emotions with different intensities
        emotions = [
            ('joy', 0.8),
            ('sadness', 0.6),
            ('joy', 0.7),  # More joy
            ('anger', 0.3)
        ]
        
        dominant_emotion, dominant_intensity = self.game.get_dominant_emotion(emotions)
        
        # Joy should be dominant (0.8 + 0.7 = 1.5 total)
        self.assertEqual(dominant_emotion, 'joy')
        
        # Total intensity should be 1.5 (not average)
        self.assertAlmostEqual(dominant_intensity, 1.5, places=1)
        
        print("✅ Get dominant emotion test passed")
    
    def test_conway_rules(self):
        """Test Conway's Game of Life rules"""
        print("🧪 Test 6: Conway's Rules")
        
        # Set up a blinker pattern (3 cells in a row)
        self.game.grid[5, 4] = True
        self.game.grid[5, 5] = True
        self.game.grid[5, 6] = True
        
        # Update the grid
        self.game.update_grid()
        
        # After one update, should become vertical
        self.assertTrue(self.game.grid[4, 5])  # Top
        self.assertTrue(self.game.grid[5, 5])  # Center
        self.assertTrue(self.game.grid[6, 5])  # Bottom
        
        # The original horizontal cells should be dead
        self.assertFalse(self.game.grid[5, 4])
        self.assertFalse(self.game.grid[5, 6])
        
        print("✅ Conway's rules test passed")
    
    def test_emotion_spreading(self):
        """Test emotion spreading between cells"""
        print("🧪 Test 7: Emotion Spreading")
        
        # Set up emotion sources
        self.game.add_emotion_source('joy', 50, 50)      # Grid (5, 5)
        self.game.add_emotion_source('sadness', 60, 50)  # Grid (6, 5)
        
        # Create a simple pattern that will definitely create a new cell
        # Add 3 cells in a row to create a blinker
        self.game.grid[5, 4] = True
        self.game.grid[5, 5] = True  # This will be overwritten by emotion source
        self.game.grid[5, 6] = True
        
        # Update the grid
        self.game.update_grid()
        
        # After Conway's rules, should have vertical pattern
        # Check that we have some alive cells and emotions
        alive_count = np.sum(self.game.grid)
        emotion_count = np.sum((self.game.emotions != None) & (self.game.intensities > 0.1))
        
        self.assertGreater(alive_count, 0)
        self.assertGreater(emotion_count, 0)
        
        print("✅ Emotion spreading test passed")
    
    def test_emotion_decay(self):
        """Test that emotions decay over time"""
        print("🧪 Test 8: Emotion Decay")
        
        # Add an emotion source
        self.game.add_emotion_source('joy', 50, 50)
        
        # Check initial intensity
        self.assertEqual(self.game.intensities[5, 5], 1.0)
        
        # Update the grid (should decay by 0.9)
        self.game.update_grid()
        
        # Check that intensity has decayed
        self.assertLess(self.game.intensities[5, 5], 1.0)
        self.assertAlmostEqual(self.game.intensities[5, 5], 0.9, places=1)
        
        print("✅ Emotion decay test passed")
    
    def test_color_rendering(self):
        """Test that colors are rendered correctly"""
        print("🧪 Test 9: Color Rendering")
        
        # Check that colors are defined
        self.assertIn('joy', COLORS)
        self.assertIn('sadness', COLORS)
        self.assertIn('anger', COLORS)
        
        # Check that colors are RGB tuples
        for emotion, color in COLORS.items():
            if emotion != 'background' and emotion != 'grid':
                self.assertEqual(len(color), 3)  # RGB
                for component in color:
                    self.assertGreaterEqual(component, 0)
                    self.assertLessEqual(component, 255)
        
        print("✅ Color rendering test passed")
    
    def test_performance(self):
        """Test that the game can handle reasonable performance"""
        print("🧪 Test 10: Performance")
        
        # Create a larger test game
        large_game = EmotionContagion(width=400, height=300, cell_size=4)
        
        # Add some random emotion sources
        for i in range(10):
            x = np.random.randint(0, 400)
            y = np.random.randint(0, 300)
            emotion = np.random.choice(['joy', 'sadness', 'anger'])
            large_game.add_emotion_source(emotion, x, y)
        
        # Time a few updates
        import time
        start_time = time.time()
        
        for i in range(10):
            large_game.update_grid()
        
        end_time = time.time()
        update_time = end_time - start_time
        
        # Should be reasonably fast (less than 1 second for 10 updates)
        self.assertLess(update_time, 1.0)
        
        print(f"✅ Performance test passed: {update_time:.3f}s for 10 updates")
    
    def test_edge_cases(self):
        """Test edge cases and boundary conditions"""
        print("🧪 Test 11: Edge Cases")
        
        # Test adding emotion source outside grid
        self.game.add_emotion_source('joy', 999, 999)
        
        # Test neighbor counting at edges
        neighbors = self.game.count_neighbors(0, 0)
        self.assertEqual(neighbors, 0)
        
        # Test empty emotion list
        dominant_emotion, dominant_intensity = self.game.get_dominant_emotion([])
        self.assertIsNone(dominant_emotion)
        self.assertEqual(dominant_intensity, 0.0)
        
        print("✅ Edge cases test passed")
    
    def test_integration(self):
        """Test the complete system integration"""
        print("🧪 Test 12: Integration")
        
        # Set up a complex scenario
        self.game.add_emotion_source('joy', 30, 30)
        self.game.add_emotion_source('sadness', 70, 30)
        self.game.add_emotion_source('anger', 50, 70)
        
        # Create a pattern that will evolve
        self.game.grid[4, 4] = True
        self.game.grid[4, 5] = True
        self.game.grid[4, 6] = True
        self.game.grid[5, 4] = True
        self.game.grid[5, 6] = True
        self.game.grid[6, 4] = True
        self.game.grid[6, 5] = True
        self.game.grid[6, 6] = True
        
        # Run several updates
        for i in range(5):
            self.game.update_grid()
        
        # Check that the system is still working
        alive_count = np.sum(self.game.grid)
        emotion_count = np.sum((self.game.emotions != None) & (self.game.intensities > 0.1))
        
        # Should have some alive cells and emotions
        self.assertGreater(alive_count, 0)
        self.assertGreater(emotion_count, 0)
        
        print(f"✅ Integration test passed: {alive_count} alive cells, {emotion_count} emotions")

def run_visual_test():
    """Run a visual test to see the game in action"""
    print("🎮 Running Visual Test...")
    print("This will open a window. Click to add emotions, press SPACE to pause.")
    print("Close the window when done.")
    
    game = EmotionContagion(width=400, height=300, cell_size=4)
    
    # Add some initial emotion sources
    game.add_emotion_source('joy', 100, 100)
    game.add_emotion_source('sadness', 300, 100)
    game.add_emotion_source('anger', 200, 200)
    
    # Run for a few seconds
    import time
    start_time = time.time()
    
    running = True
    while running and (time.time() - start_time) < 10:  # Run for 10 seconds
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.MOUSEBUTTONDOWN:
                x, y = pygame.mouse.get_pos()
                import random
                emotions = ['joy', 'sadness', 'anger', 'fear', 'disgust']
                emotion = random.choice(emotions)
                game.add_emotion_source(emotion, x, y)
        
        game.update_grid()
        game.render()
        game.render_stats()
        pygame.display.flip()
        game.clock.tick(30)  # 30 FPS
    
    pygame.quit()
    print("✅ Visual test completed")

if __name__ == "__main__":
    print("🧪 Starting Comprehensive Test Suite")
    print("=" * 50)
    
    # Run unit tests
    unittest.main(verbosity=2, exit=False)
    
    print("\n" + "=" * 50)
    print("🎮 Starting Visual Test")
    
    # Run visual test
    run_visual_test()
    
    print("\n🎉 All tests completed!") 