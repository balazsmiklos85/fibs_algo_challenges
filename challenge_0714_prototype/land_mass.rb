# frozen_string_literal: true

class LandMass
  attr_reader :start_index, :end_index

  def initialize(index)
    @start_index = index
    @end_index = index
  end

  def adjacent?(other)
    other.start_index.between?(start_index, end_index) ||
      other.end_index.between?(start_index, end_index)
  end

  def any_adjacent?(other_land_masses)
    other_land_masses.any? { |other_land_mass| adjacent? other_land_mass }
  end

  def increase!
    @end_index += 1
    self
  end

  def size
    end_index - start_index + 1
  end

  def to_s
    "<#{start_index}..#{end_index}>"
  end
end
