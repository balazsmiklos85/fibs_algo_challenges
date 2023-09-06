#!/usr/bin/ruby

class Assignment
  attr_reader :intern, :team

  def initialize(intern, team)
    @intern = intern
    @team = team
  end

  def conflicts?(other)
    @intern == other.intern || @team == other.team
  end

  def score(preferences_of_interns, preferences_of_teams)
    intern_score = preferences_of_interns[@intern].index @team
    team_score = preferences_of_teams[@team].index @intern
    intern_score + team_score
  end

  def to_s
    "[#{@intern}, #{@team}]"
  end
end

class Assigner
  def initialize(team_preferences, intern_preferences)
    @team_preferences = team_preferences
    @intern_preferences = intern_preferences
  end

  # O(n!)
  def solve_with_brute_force
    team_count = @intern_preferences.length - 1
    team_indices = Array(0..team_count)
    team_assigments = team_indices.permutation(3)
    # raw: team_assignment[intern_id] = team_id
    result = team_assigments.map { |raw| to_assignment_objects raw }
                            .min_by { |assignments| score assignments }
    print_result result
  end

  # O(n^3)
  def solve_with_greedy
    possible_assignments = generate_possible_assignments_sorted_by_score
    result = []
    until possible_assignments.empty?
      best_assignment = possible_assignments.first
      result << best_assignment
      possible_assignments.reject! do |assignment|
        assignment.conflicts?(best_assignment)
      end
    end
    print_result result
  end

  private

  def generate_possible_assignments_sorted_by_score
    possible_assignments = []
    (0..(@intern_preferences.length - 1)).each do |team_index|
      (0..(@team_preferences.length - 1)).each do |intern_index|
        possible_assignments << Assignment.new(intern_index, team_index)
      end
    end
    possible_assignments.sort_by do |assignment|
      assignment.score @team_preferences, @intern_preferences
    end
  end

  def print_result(assignments)
    score = assignments.map { |assignment| assignment.score @team_preferences, @intern_preferences }
                       .reduce(:+)
    puts "Score: #{score}"
    puts "[\n  #{assignments.join(",\n  ")}\n]"
  end

  def score(assignments)
    scores = assignments.map do |assignment|
      assignment.score @team_preferences, @intern_preferences
    end
    scores.reduce(:+)
  end

  def to_assignment_objects(team_assignment)
    team_assignment.map.with_index do |team, intern|
      Assignment.new intern, team
    end
  end
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

puts 'Solution with brute force:'
Assigner.new(interns_input, teams_input)
        .solve_with_brute_force

puts 'Solution with greedy algorithm:'
Assigner.new(interns_input, teams_input)
        .solve_with_greedy
