# frozen_string_literal: true

require_relative 'land_mass'

class LandMassFactory
  LAND = 0

  attr_reader :land_masses

  def initialize(values)
    @land_masses = []
    current_land_mass = nil
    values.each_with_index do |value, index|
      current_land_mass = incorporate_land_value index, value, current_land_mass
    end
  end

  private

  def incorporate_land_value(index, value, current_land_mass)
    return unless value == LAND
    return store_new_land_mass index if current_land_mass.nil?

    current_land_mass.increase!
  end

  def store_new_land_mass(index)
    result = LandMass.new index
    @land_masses << result
    result
  end
end
