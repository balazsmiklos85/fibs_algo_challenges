# frozen_string_literal: true

class Island
  attr_reader :area, :land_masses

  def initialize(land_mass)
    @land_masses = []
    @area = 0
    return if land_mass.nil?

    @land_masses << land_mass
    @area += land_mass.size
  end

  def adjacent?(other)
    @land_masses.any? { |land_mass| land_mass.any_adjacent? other.land_masses }
  end

  def merge!(other)
    @land_masses += other.land_masses
    @area += other.area
    self
  end

  def incorporate!(others)
    @land_masses = []
    others.each { |other| merge! other }
    self
  end

  def to_s
    "I{#{area};[#{land_masses.join ', '}]}"
  end
end
