Feature: Recipes

  Background:
    Given a running server

  Scenario: Root page is redirected to recipe list
    When root page is loaded
    Then I am redirected to recipe list

  Scenario: If we load recipes they are present
    When recipes are loaded
    Then 6 recipes are shown

  Scenario: If we add a recipe and load recipes there are more
    When a recipe is added
    And recipes are loaded
    Then 7 recipes are shown
