#!/usr/bin/ruby

class Assignment
  def initialize(id, team)
    @id = id
    @team = team
  end

  def score(preferences_of_interns, preferences_of_teams)
    intern_score = preferences_of_interns[@id].index @team
    team_score = preferences_of_teams[@team].index @id
    intern_score + team_score
  end

  def to_s
    "[#{@id}, #{@team}]"
  end
end

class Assigner
  def initialize(team_preferences, intern_preferences)
    @team_preferences = team_preferences
    @intern_preferences = intern_preferences
  end

  def solve_with_brute_force
    team_count = @intern_preferences.length - 1
    team_indices = Array(0..team_count)
    team_assigments = team_indices.permutation(3)
    team_assigments.map { |raw_assignments| to_assignments raw_assignments }
                   .min_by { |assignments| calculate_score assignments }
  end

  private

  def calculate_score(assignments)
    scores = assignments.map do |assignment|
      assignment.score @team_preferences, @intern_preferences
    end
    scores.reduce(:+)
  end

  def to_assignments(team_assignment)
    team_assignment.map.with_index do |team, intern|
      Assignment.new intern, team
    end
  end
end

def print_result(interns)
  puts "[\n  #{interns.join(",\n  ")}\n]"
end

interns_input = [
  [0, 1, 2],
  [1, 0, 2],
  [1, 2, 0]
]

teams_input = [
  [2, 1, 0],
  [1, 2, 0],
  [0, 2, 1]
]

interns = Assigner.new(interns_input, teams_input)
                  .solve_with_brute_force
print_result interns
