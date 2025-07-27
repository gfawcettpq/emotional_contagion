Feature: Emotion Spreading on Grid
  As a simulation user
  I want emotions to spread between grid cells
  So that I can observe realistic emotional contagion

  Background:
    Given a 10x10 emotion grid
    And the following emotion types are available:
      | emotion | decay_rate | spread_rate | max_intensity |
      | Joy     | 0.02       | 0.1         | 1.0           |
      | Sadness | 0.015      | 0.08        | 1.0           |
      | Anger   | 0.03       | 0.15        | 1.0           |

  Scenario: Joy spreads to neighboring cells
    Given cell (5,5) has Joy intensity 1.0
    When the simulation runs for 1 step
    Then neighboring cells should have Joy intensity greater than 0
    And cell (5,5) should have Joy intensity less than 1.0 due to decay

  Scenario: Multiple emotions can coexist
    Given cell (5,5) has Joy intensity 0.8
    And cell (5,5) has Sadness intensity 0.3
    When the simulation runs for 1 step
    Then cell (5,5) should have both Joy and Sadness
    And both emotions should have decayed

  Scenario: Emotions spread at different rates
    Given cell (5,5) has Anger intensity 1.0
    And cell (7,7) has Joy intensity 1.0
    When the simulation runs for 3 steps
    Then Anger should have spread further than Joy
    Because Anger has a higher spread rate