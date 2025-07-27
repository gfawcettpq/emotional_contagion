Feature: Rule Engine Processing
  As a simulation designer
  I want to create complex emotion interaction rules
  So that I can model sophisticated emotional scenarios

  Background:
    Given the rule engine is initialized
    And a 15x15 emotion grid exists

  Scenario: Simple proximity rule triggers
    Given a rule "joy_spreader":
      | trigger | NearEmotion | emotion=Joy, min_intensity=0.5, distance=2 |
      | effect  | AddEmotion  | emotion=Love, amount=0.3                    |
    And an entity at position (7,7) with Joy intensity 0.8
    And an entity at position (8,8) with Joy intensity 0.6
    When the rule engine processes all rules
    Then the entity at (8,8) should have Love intensity 0.3

  Scenario: Complex AND trigger combination
    Given a rule "anxiety_amplifier":
      | trigger | And | [NearEntity(Source, 3), EmotionThreshold(Anxiety, 0.2, Greater)] |
      | effect  | MultiplyEmotion | emotion=Anxiety, multiplier=2.0 |
    And a Source entity at position (5,5)
    And an entity at position (7,7) with Anxiety intensity 0.3
    When the rule engine processes all rules
    Then the entity at (7,7) should have Anxiety intensity 0.6

  Scenario: Rule priority ordering
    Given these rules with priorities:
      | name        | priority | trigger              | effect                    |
      | high_prio   | 100      | EmotionThreshold(Joy, 0.5, Greater) | SetEmotion(Joy, 1.0)     |
      | low_prio    | 10       | EmotionThreshold(Joy, 0.5, Greater) | SetEmotion(Joy, 0.1)     |
    And an entity with Joy intensity 0.7
    When the rule engine processes all rules
    Then the entity should have Joy intensity 1.0
    Because higher priority rules execute first

  Scenario: Custom Lua rule execution
    Given a custom Lua rule "embarrassment_spreader":
      """
      function evaluate_trigger(entity, entities, grid)
          local anger = entity.emotions["Anger"] or 0
          local embarrassment = entity.emotions["Embarrassment"] or 0
          return anger > 0.5 and embarrassment > 0.2
      end
      
      function apply_effect(entity, entities, grid)
          entity.emotions["Embarrassment"] = (entity.emotions["Embarrassment"] or 0) * 1.5
      end
      """
    And an entity with Anger intensity 0.6 and Embarrassment intensity 0.3
    When the rule engine processes all rules
    Then the entity should have Embarrassment intensity 0.45