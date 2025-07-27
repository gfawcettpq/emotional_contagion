Feature: Entity Interactions
  As a simulation user
  I want entities to interact with each other and the grid
  So that I can model complex emotional scenarios

  Background:
    Given a 20x20 emotion grid
    And the simulation is running

  Scenario: Person entity affects grid emotions
    Given a Person entity at position (10,10)
    And the Person has Joy intensity 0.9
    When the simulation runs for 2 steps
    Then the grid cell (10,10) should have increased Joy
    And neighboring grid cells should show Joy spreading

  Scenario: Source entity continuously generates emotions
    Given a Source entity at position (5,5)
    And the Source generates Anxiety with intensity 0.7
    When the simulation runs for 5 steps
    Then cell (5,5) should maintain high Anxiety levels
    And Anxiety should spread outward in a radius

  Scenario: Modifier entity affects nearby emotion spread rates
    Given an Anchor entity at position (8,8)
    And the Anchor has SpreadRateModifier for Joy with multiplier 2.0
    And Joy is present at position (8,8)
    When the simulation runs for 3 steps
    Then Joy should spread faster near the Anchor
    Compared to Joy spreading elsewhere on the grid

  Scenario: Moving entities create emotion trails
    Given a Person entity with Random movement pattern
    And the Person starts at position (0,0) with Joy intensity 1.0
    When the simulation runs for 10 steps
    Then multiple grid cells should show Joy traces
    And the entity should have moved to a different position