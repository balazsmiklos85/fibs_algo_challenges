# frozen_string_literal: true

class LargestRectangleBuilder
  def initialize(buildings)
    @buildings = buildings
    @heights = @buildings.sort
                         .uniq
    @largest_rectangle = 0
  end

  def build
    @heights.each { |height| build_for height }
    @largest_rectangle
  end

  private

  def build_for(height)
    rectangle = 0
    @buildings.each do |building|
      rectangle = extend_or_close(building, height, rectangle)
      @largest_rectangle = rectangle if rectangle > @largest_rectangle
    end
  end

  def extend_or_close(building, height, rectangle)
    building >= height ? rectangle + height : 0
  end
end

buildings = [1, 3, 3, 2, 4, 1, 5, 3, 2]

puts LargestRectangleBuilder.new(buildings).build
