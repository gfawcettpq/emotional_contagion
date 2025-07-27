#!/usr/bin/env python3
"""
🌊 Emotion Contagion - Conway's Game of Life
A beautiful visual simulation of emotion spreading through cellular automata
"""

import pygame
import numpy as np
import random
import time
from typing import List, Tuple, Optional
from dataclasses import dataclass

# Nord color palette
COLORS = {
    'background': (46, 52, 64),      # #2e3440
    'panel_bg': (59, 66, 82),       # #3b4252
    'panel_border': (67, 76, 94),   # #434c5e
    'text_primary': (229, 233, 240), # #e5e9f0
    'text_secondary': (180, 184, 195), # #b4b8c5
    'accent': (136, 192, 208),      # #88c0d0
    'joy': (163, 190, 140),         # #a3be8c - Green
    'sadness': (94, 129, 172),      # #5e81ac - Blue
    'anger': (191, 97, 106),        # #bf616a - Red
    'fear': (180, 142, 173),        # #b48ead - Purple
    'disgust': (235, 203, 139),     # #ebcb8b - Yellow
    'anxiety': (163, 190, 140),     # #a3be8c - Light green
    'love': (216, 222, 233),        # #d8dee9 - White
    'envy': (94, 129, 172),         # #5e81ac - Dark blue
    'embarrassment': (180, 142, 173) # #b48ead - Purple
}

@dataclass
class Character:
    name: str
    emotion: str
    color: Tuple[int, int, int]
    symbol: str
    description: str

CHARACTERS = [
    Character("Joy", "joy", COLORS['joy'], "😊", "Spreads happiness and positive energy"),
    Character("Sadness", "sadness", COLORS['sadness'], "😢", "Creates melancholy and reflection"),
    Character("Anger", "anger", COLORS['anger'], "😡", "Generates intense, aggressive energy"),
    Character("Fear", "fear", COLORS['fear'], "😨", "Creates anxiety and uncertainty"),
    Character("Disgust", "disgust", COLORS['disgust'], "🤢", "Spreads aversion and rejection"),
    Character("Anxiety", "anxiety", COLORS['anxiety'], "😰", "Creates nervous tension"),
    Character("Love", "love", COLORS['love'], "💕", "Spreads warmth and connection"),
    Character("Envy", "envy", COLORS['envy'], "😒", "Creates jealousy and resentment"),
    Character("Embarrassment", "embarrassment", COLORS['embarrassment'], "😳", "Generates awkwardness")
]

class EmotionContagion:
    def __init__(self, width=1920, height=1080, cell_size=6):
        pygame.init()
        self.width = width
        self.height = height
        self.cell_size = cell_size
        self.grid_width = width // cell_size
        self.grid_height = height // cell_size
        
        # Initialize display
        self.screen = pygame.display.set_mode((width, height), pygame.FULLSCREEN)
        pygame.display.set_caption("🌊 Emotion Contagion - Conway's Game of Life")
        
        # Initialize grid
        self.grid = np.zeros((self.grid_height, self.grid_width), dtype=bool)
        self.emotions = np.full((self.grid_height, self.grid_width), None, dtype=object)
        self.intensities = np.zeros((self.grid_height, self.grid_width))
        
        # Game state
        self.paused = False
        self.update_count = 0
        self.clock = pygame.time.Clock()
        self.font_large = pygame.font.Font(None, 48)
        self.font_medium = pygame.font.Font(None, 24)
        self.font_small = pygame.font.Font(None, 18)
        
        # Performance tracking
        self.fps = 60
        self.last_update = pygame.time.get_ticks()
        
        # UI state
        self.show_help = False
        self.dragging_character = None
        self.drag_start = None
        self.drag_offset = (0, 0)
        
        # Panel dimensions
        self.panel_width = 300
        self.panel_height = 200
        self.toolbox_width = 350
        self.toolbox_height = 400
        
        # Logging
        self.log_events = True
        self.event_log = []
        
        print(f"🎮 Game initialized: {self.grid_width}x{self.grid_height} grid, {self.cell_size}px cells")
        
    def log_event(self, message: str):
        """Log an event with timestamp"""
        if self.log_events:
            timestamp = time.strftime("%H:%M:%S")
            log_entry = f"[{timestamp}] {message}"
            print(log_entry)
            self.event_log.append(log_entry)
            if len(self.event_log) > 50:
                self.event_log = self.event_log[-50:]
        
    def add_emotion_source(self, emotion: str, x: int, y: int):
        """Add an emotion source at grid position"""
        grid_x = x // self.cell_size
        grid_y = y // self.cell_size
        
        if 0 <= grid_x < self.grid_width and 0 <= grid_y < self.grid_height:
            self.grid[grid_y, grid_x] = True
            self.emotions[grid_y, grid_x] = emotion
            self.intensities[grid_y, grid_x] = 1.0
            self.log_event(f"😊 Added {emotion} source at ({x}, {y}) -> grid({grid_x}, {grid_y})")
        else:
            self.log_event(f"❌ Failed to add emotion at ({x}, {y}) - out of bounds")
            
    def count_neighbors(self, x: int, y: int) -> int:
        """Count live neighbors for Conway's Game of Life"""
        count = 0
        for dy in range(-1, 2):
            for dx in range(-1, 2):
                if dx == 0 and dy == 0:
                    continue
                nx, ny = x + dx, y + dy
                if (0 <= nx < self.grid_width and 
                    0 <= ny < self.grid_height and 
                    self.grid[ny, nx]):
                    count += 1
        return count
    
    def get_neighbor_emotions(self, x: int, y: int) -> List[Tuple[str, float]]:
        """Get emotions from neighboring cells"""
        emotions = []
        for dy in range(-1, 2):
            for dx in range(-1, 2):
                if dx == 0 and dy == 0:
                    continue
                nx, ny = x + dx, y + dy
                if (0 <= nx < self.grid_width and 
                    0 <= ny < self.grid_height and 
                    self.grid[ny, nx] and
                    self.emotions[ny, nx] is not None and 
                    self.intensities[ny, nx] > 0.1):
                    emotions.append((self.emotions[ny, nx], self.intensities[ny, nx]))
        return emotions
    
    def get_dominant_emotion(self, emotions: List[Tuple[str, float]]) -> Tuple[str, float]:
        """Find the dominant emotion from neighbors"""
        if not emotions:
            return None, 0.0
            
        emotion_counts = {}
        total_intensity = 0.0
        
        for emotion, intensity in emotions:
            if emotion not in emotion_counts:
                emotion_counts[emotion] = 0.0
            emotion_counts[emotion] += intensity
            total_intensity += intensity
            
        if not emotion_counts:
            return None, 0.0
            
        dominant_emotion = max(emotion_counts.items(), key=lambda x: x[1])
        return dominant_emotion[0], dominant_emotion[1]
    
    def update_grid(self):
        """Update the grid using Conway's Game of Life rules with emotion spreading"""
        new_grid = np.zeros_like(self.grid)
        new_emotions = np.full_like(self.emotions, None)
        new_intensities = np.zeros_like(self.intensities)
        
        births = 0
        deaths = 0
        emotion_spreads = 0
        
        for y in range(self.grid_height):
            for x in range(self.grid_width):
                neighbors = self.count_neighbors(x, y)
                current_alive = self.grid[y, x]
                
                # Conway's Game of Life rules - EXACTLY like the Rust version
                if current_alive:
                    if neighbors < 2 or neighbors > 3:
                        alive = False
                        deaths += 1
                    else:
                        alive = True
                else:
                    if neighbors == 3:
                        alive = True
                        births += 1
                    else:
                        alive = False
                
                # Emotion spreading
                emotion = self.emotions[y, x]
                intensity = self.intensities[y, x] * 0.9  # Decay
                
                if alive:
                    neighbor_emotions = self.get_neighbor_emotions(x, y)
                    if neighbor_emotions:
                        dominant_emotion, dominant_intensity = self.get_dominant_emotion(neighbor_emotions)
                        if dominant_emotion:
                            emotion = dominant_emotion
                            intensity = min(1.0, intensity + dominant_intensity * 0.3)
                            emotion_spreads += 1
                
                new_grid[y, x] = alive
                new_emotions[y, x] = emotion
                new_intensities[y, x] = intensity
        
        self.grid = new_grid
        self.emotions = new_emotions
        self.intensities = new_intensities
        
        if births > 0 or deaths > 0 or emotion_spreads > 0:
            self.log_event(f"🔄 Conway Update #{self.update_count}: {births} births, {deaths} deaths, {emotion_spreads} emotion spreads")
        
        self.update_count += 1
    
    def draw_panel(self, x: int, y: int, width: int, height: int, title: str, content: List[str], font=None):
        """Draw a nord-themed panel"""
        if font is None:
            font = self.font_small
            
        # Panel background
        pygame.draw.rect(self.screen, COLORS['panel_bg'], (x, y, width, height))
        pygame.draw.rect(self.screen, COLORS['panel_border'], (x, y, width, height), 2)
        
        # Title
        title_surface = font.render(title, True, COLORS['accent'])
        self.screen.blit(title_surface, (x + 10, y + 10))
        
        # Content
        content_y = y + 40
        for line in content:
            if content_y + 20 < y + height:
                text_surface = font.render(line, True, COLORS['text_primary'])
                self.screen.blit(text_surface, (x + 10, content_y))
                content_y += 20
    
    def draw_toolbox(self):
        """Draw the character toolbox"""
        x = self.width - self.toolbox_width - 20
        y = 20
        
        # Toolbox background
        pygame.draw.rect(self.screen, COLORS['panel_bg'], (x, y, self.toolbox_width, self.toolbox_height))
        pygame.draw.rect(self.screen, COLORS['panel_border'], (x, y, self.toolbox_width, self.toolbox_height), 2)
        
        # Title
        title_surface = self.font_medium.render("🎭 Emotion Characters", True, COLORS['accent'])
        self.screen.blit(title_surface, (x + 10, y + 10))
        
        # Character grid
        char_x = x + 20
        char_y = y + 50
        chars_per_row = 3
        
        for i, character in enumerate(CHARACTERS):
            col = i % chars_per_row
            row = i // chars_per_row
            
            char_pos_x = char_x + col * 100
            char_pos_y = char_y + row * 80
            
            # Character box
            char_rect = pygame.Rect(char_pos_x, char_pos_y, 80, 60)
            pygame.draw.rect(self.screen, COLORS['panel_border'], char_rect)
            pygame.draw.rect(self.screen, character.color, char_rect, 2)
            
            # Character symbol (larger and more visible)
            symbol_surface = self.font_large.render(character.symbol, True, character.color)
            symbol_rect = symbol_surface.get_rect(center=(char_pos_x + 40, char_pos_y + 20))
            self.screen.blit(symbol_surface, symbol_rect)
            
            # Character name
            name_surface = self.font_small.render(character.name, True, COLORS['text_primary'])
            name_rect = name_surface.get_rect(center=(char_pos_x + 40, char_pos_y + 45))
            self.screen.blit(name_surface, name_rect)
    
    def draw_dragging_character(self):
        """Draw the character being dragged"""
        if self.dragging_character and self.drag_start:
            mouse_pos = pygame.mouse.get_pos()
            drag_x = mouse_pos[0] - self.drag_offset[0]
            drag_y = mouse_pos[1] - self.drag_offset[1]
            
            # Draw character being dragged
            char_rect = pygame.Rect(drag_x, drag_y, 80, 60)
            pygame.draw.rect(self.screen, COLORS['panel_bg'], char_rect)
            pygame.draw.rect(self.screen, self.dragging_character.color, char_rect, 3)
            
            # Character symbol
            symbol_surface = self.font_large.render(self.dragging_character.symbol, True, self.dragging_character.color)
            symbol_rect = symbol_surface.get_rect(center=(drag_x + 40, drag_y + 20))
            self.screen.blit(symbol_surface, symbol_rect)
            
            # Character name
            name_surface = self.font_small.render(self.dragging_character.name, True, COLORS['text_primary'])
            name_rect = name_surface.get_rect(center=(drag_x + 40, drag_y + 45))
            self.screen.blit(name_surface, name_rect)
    
    def draw_info_panel(self):
        """Draw the top-left info panel"""
        content = [
            "🌊 Conway's Game of Life",
            "with Emotion Contagion",
            "",
            "Click and drag characters",
            "from the toolbox to",
            "add them to the grid.",
            "",
            "Watch emotions spread",
            "through the cellular",
            "automata patterns."
        ]
        self.draw_panel(20, 20, self.panel_width, self.panel_height, "ℹ️ Information", content)
    
    def draw_rules_panel(self):
        """Draw the bottom-left rules panel"""
        content = [
            "Conway's Game of Life Rules:",
            "",
            "• Any live cell with < 2 neighbors dies",
            "• Any live cell with 2-3 neighbors lives",
            "• Any live cell with > 3 neighbors dies",
            "• Any dead cell with exactly 3 neighbors",
            "  becomes alive",
            "",
            "Emotion Spreading:",
            "• Live cells inherit emotions from",
            "  neighboring cells",
            "• Emotions decay over time",
            "• Multiple emotions compete for",
            "  dominance"
        ]
        self.draw_panel(20, self.height - self.panel_height - 20, self.panel_width, self.panel_height + 50, "📋 Rules", content)
    
    def draw_stats_panel(self):
        """Draw the bottom-right stats panel"""
        alive_count = np.sum(self.grid)
        emotion_count = np.sum((self.emotions != None) & (self.intensities > 0.1))
        
        content = [
            f"Alive Cells: {alive_count}",
            f"Emotion Sources: {emotion_count}",
            f"Updates: {self.update_count}",
            f"FPS: {int(self.clock.get_fps())}",
            f"Status: {'PAUSED' if self.paused else 'RUNNING'}",
            "",
            "Controls:",
            "SPACE - Pause/Resume",
            "R - Randomize",
            "C - Clear",
            "H - Toggle Help",
            "ESC - Exit"
        ]
        self.draw_panel(self.width - self.panel_width - 20, self.height - self.panel_height - 20, self.panel_width, self.panel_height + 50, "📊 Statistics", content)
    
    def draw_help_overlay(self):
        """Draw the help overlay"""
        if not self.show_help:
            return
            
        # Semi-transparent overlay
        overlay = pygame.Surface((self.width, self.height))
        overlay.set_alpha(128)
        overlay.fill(COLORS['background'])
        self.screen.blit(overlay, (0, 0))
        
        # Help content
        help_content = [
            "🎮 EMOTION CONTAGION - CONWAY'S GAME OF LIFE",
            "",
            "HOW TO PLAY:",
            "1. Click and drag characters from the toolbox",
            "2. Drop them anywhere on the grid",
            "3. Watch emotions spread through the cellular automata",
            "",
            "CONTROLS:",
            "• SPACE - Pause/Resume simulation",
            "• R - Randomize the grid",
            "• C - Clear the grid",
            "• H - Toggle this help screen",
            "• ESC - Exit the game",
            "",
            "EMOTIONS:",
            "• Each character represents an emotion",
            "• Emotions spread to neighboring cells",
            "• Multiple emotions compete for dominance",
            "• Emotions decay over time",
            "",
            "CONWAY'S RULES:",
            "• Live cells with 2-3 neighbors survive",
            "• Dead cells with exactly 3 neighbors become alive",
            "• All other cells die or stay dead"
        ]
        
        # Draw help panel
        help_width = 600
        help_height = 500
        help_x = (self.width - help_width) // 2
        help_y = (self.height - help_height) // 2
        
        pygame.draw.rect(self.screen, COLORS['panel_bg'], (help_x, help_y, help_width, help_height))
        pygame.draw.rect(self.screen, COLORS['accent'], (help_x, help_y, help_width, help_height), 3)
        
        # Help title
        title_surface = self.font_large.render("❓ HELP", True, COLORS['accent'])
        self.screen.blit(title_surface, (help_x + 20, help_y + 20))
        
        # Help content
        content_y = help_y + 60
        for line in help_content:
            if content_y + 20 < help_y + help_height:
                text_surface = self.font_small.render(line, True, COLORS['text_primary'])
                self.screen.blit(text_surface, (help_x + 20, content_y))
                content_y += 20
    
    def render(self):
        """Render the grid with emotion colors"""
        # Clear screen
        self.screen.fill(COLORS['background'])
        
        # Draw grid cells
        for y in range(self.grid_height):
            for x in range(self.grid_width):
                if self.grid[y, x]:
                    pixel_x = x * self.cell_size
                    pixel_y = y * self.cell_size
                    
                    # Get color based on emotion
                    if (self.emotions[y, x] and 
                        self.intensities[y, x] > 0.1 and 
                        self.emotions[y, x] in COLORS):
                        color = list(COLORS[self.emotions[y, x]])
                        # Adjust brightness based on intensity
                        brightness = 0.3 + (self.intensities[y, x] * 0.7)
                        color = [int(c * brightness) for c in color]
                    else:
                        color = COLORS['panel_border']
                    
                    pygame.draw.rect(self.screen, color, 
                                   (pixel_x, pixel_y, self.cell_size, self.cell_size))
        
        # Draw UI panels
        self.draw_info_panel()
        self.draw_toolbox()
        self.draw_rules_panel()
        self.draw_stats_panel()
        
        # Draw dragging character if active
        self.draw_dragging_character()
        
        # Draw help overlay if active
        self.draw_help_overlay()
    
    def get_character_at_pos(self, x: int, y: int) -> Optional[Character]:
        """Get character at mouse position in toolbox"""
        toolbox_x = self.width - self.toolbox_width - 20
        toolbox_y = 20
        
        if (toolbox_x <= x <= toolbox_x + self.toolbox_width and 
            toolbox_y + 50 <= y <= toolbox_y + self.toolbox_height):
            
            # Calculate character position
            rel_x = x - toolbox_x - 20
            rel_y = y - toolbox_y - 50
            
            col = rel_x // 100
            row = (rel_y // 80)
            
            char_index = row * 3 + col
            if 0 <= char_index < len(CHARACTERS):
                return CHARACTERS[char_index]
        
        return None
    
    def handle_events(self):
        """Handle pygame events"""
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                return False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_SPACE:
                    self.paused = not self.paused
                    self.log_event(f"⏯️ Game {'PAUSED' if self.paused else 'RESUMED'}")
                elif event.key == pygame.K_r:
                    self.randomize_grid()
                    self.log_event("🎲 Grid randomized")
                elif event.key == pygame.K_c:
                    self.clear_grid()
                    self.log_event("🗑️ Grid cleared")
                elif event.key == pygame.K_h:
                    self.show_help = not self.show_help
                    self.log_event(f"❓ Help {'ON' if self.show_help else 'OFF'}")
                elif event.key == pygame.K_ESCAPE:
                    return False
            elif event.type == pygame.MOUSEBUTTONDOWN:
                if event.button == 1:  # Left click
                    character = self.get_character_at_pos(event.pos[0], event.pos[1])
                    if character:
                        self.dragging_character = character
                        self.drag_start = event.pos
                        # Calculate offset from character center
                        toolbox_x = self.width - self.toolbox_width - 20
                        toolbox_y = 20
                        rel_x = event.pos[0] - toolbox_x - 20
                        rel_y = event.pos[1] - toolbox_y - 50
                        col = rel_x // 100
                        row = (rel_y // 80)
                        char_pos_x = toolbox_x + 20 + col * 100 + 40
                        char_pos_y = toolbox_y + 50 + row * 80 + 30
                        self.drag_offset = (event.pos[0] - char_pos_x, event.pos[1] - char_pos_y)
                        self.log_event(f"🎭 Started dragging {character.name}")
            elif event.type == pygame.MOUSEBUTTONUP:
                if event.button == 1 and self.dragging_character:
                    # Add character to grid
                    grid_x = event.pos[0] // self.cell_size
                    grid_y = event.pos[1] // self.cell_size
                    
                    if (0 <= grid_x < self.grid_width and 
                        0 <= grid_y < self.grid_height):
                        self.add_emotion_source(self.dragging_character.emotion, 
                                             event.pos[0], event.pos[1])
                        self.log_event(f"🎭 Dropped {self.dragging_character.name} at ({event.pos[0]}, {event.pos[1]})")
                    
                    self.dragging_character = None
                    self.drag_start = None
                    self.drag_offset = (0, 0)
        
        return True
    
    def get_character_at_pos(self, x: int, y: int) -> Optional[Character]:
        """Get character at mouse position in toolbox"""
        toolbox_x = self.width - self.toolbox_width - 20
        toolbox_y = 20
        
        if (toolbox_x <= x <= toolbox_x + self.toolbox_width and 
            toolbox_y + 50 <= y <= toolbox_y + self.toolbox_height):
            
            # Calculate character position
            rel_x = x - toolbox_x - 20
            rel_y = y - toolbox_y - 50
            
            col = rel_x // 100
            row = (rel_y // 80)
            
            char_index = row * 3 + col
            if 0 <= char_index < len(CHARACTERS):
                return CHARACTERS[char_index]
        
        return None
    
    def randomize_grid(self):
        """Randomize the grid with some emotion sources"""
        self.grid = np.random.choice([True, False], size=self.grid.shape, p=[0.3, 0.7])
        self.emotions = np.full_like(self.emotions, None)
        self.intensities = np.zeros_like(self.intensities)
        
        # Add some random emotion sources
        for _ in range(10):
            emotions = ['joy', 'sadness', 'anger', 'fear', 'disgust']
            emotion = random.choice(emotions)
            self.add_random_emotion_source(emotion)
    
    def clear_grid(self):
        """Clear the grid"""
        self.grid = np.zeros_like(self.grid)
        self.emotions = np.full_like(self.emotions, None)
        self.intensities = np.zeros_like(self.intensities)
        self.update_count = 0
    
    def add_random_emotion_source(self, emotion: str):
        """Add a random emotion source"""
        x = random.randint(0, self.width - 1)
        y = random.randint(0, self.height - 1)
        self.add_emotion_source(emotion, x, y)
    
    def run(self):
        """Main game loop"""
        print("🌊 Emotion Contagion - Conway's Game of Life")
        print("🎮 Controls:")
        print("   🖱️ Click and drag characters from toolbox")
        print("   ⏯️ SPACE: Pause/Resume")
        print("   🎲 R: Randomize grid")
        print("   🗑️ C: Clear grid")
        print("   ❓ H: Toggle Help")
        print("   🚪 ESC: Exit")
        
        # Add some initial emotion sources
        self.add_emotion_source('joy', 100, 100)
        self.add_emotion_source('sadness', 300, 100)
        self.add_emotion_source('anger', 200, 200)
        
        running = True
        while running:
            running = self.handle_events()
            
            if not self.paused:
                self.update_grid()
            
            self.render()
            pygame.display.flip()
            self.clock.tick(self.fps)
        
        pygame.quit()

if __name__ == "__main__":
    game = EmotionContagion()
    game.run() 