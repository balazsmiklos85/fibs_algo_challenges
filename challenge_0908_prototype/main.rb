#!/usr/bin/ruby

class Intern
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

def print_result(interns)
  puts "[\n  #{interns.join(",\n  ")}\n]"
end

def solve_with_brute_force(team_preferences, intern_preferences)
  team_count = intern_preferences.length - 1
  Array(0..team_count).permutation(3)
                      .map do |team_assignment|
                        team_assignment.map.with_index do |team, intern|
                          Intern.new intern, team
                        end
                      end
                      .min_by do |interns|
                        interns.map do |intern|
                          intern.score team_preferences, intern_preferences
                        end
                               .reduce(:+)
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

interns = solve_with_brute_force interns_input, teams_input
print_result interns
