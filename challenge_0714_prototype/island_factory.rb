# frozen_string_literal: true

require_relative 'island'

class IslandFactory
  attr_reader :islands

  def initialize(land_masses)
    @islands = land_masses.map { |land_mass| Island.new land_mass }
  end

  def self.dummy_island
    Island.new nil
  end
end
