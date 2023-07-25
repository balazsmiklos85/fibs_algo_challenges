# frozen_string_literal: true

def largest_rectangle_under_skyline(buildings)
  largest_rectangle = 0
  buildings.min.upto(buildings.max) do |height|
    rectangle_size = 0
    buildings.each do |building|
      rectangle_size = building >= height ? rectangle + height : 0
      largest_rectangle = rectangle if rectangle > largest_rectangle
    end
  end
  largest_rectangle
end

input = [1, 3, 3, 2, 4, 1, 5, 3, 2]

puts largest_rectangle_under_skyline(input)
