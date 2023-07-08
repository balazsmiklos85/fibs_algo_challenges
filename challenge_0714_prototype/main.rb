# frozen_string_literal: true

require_relative 'island_factory'
require_relative 'island_merger'
require_relative 'land_mass_factory'

matrix = [
  [0, 1, 1],
  [0, 0, 1],
  [1, 1, 0]
]

islands_by_lines = matrix.map { |row| LandMassFactory.new(row).land_masses }
                         .map { |land_masses| IslandFactory.new(land_masses) }
                         .map(&:islands)
complete_islands = IslandMerger.new(islands_by_lines)
                               .merge

puts complete_islands.max_by(&:area).area
