# frozen_string_literal: true

class IslandMerger
  def initialize(islands_by_lines)
    @islands_by_lines = islands_by_lines
  end

  def merge
    @previous_islands = []
    @complete_islands = []
    @islands_by_lines.each do |islands|
      islands.each { |island| merge_previous_adjacent_islands island }
      @next_islands = []
      @previous_islands.each { |previous| incorporate_into previous, islands }
      @previous_islands = @next_islands + islands
    end
    @complete_islands + @previous_islands
  end

  private

  def incorporate_into(previous, islands)
    adjacent_islands = islands.select { |island| island.adjacent? previous }
    islands.reject! { |island| island.adjacent? previous }
    if adjacent_islands.empty?
      @complete_islands << previous
    else
      new_islands_become_the_previous_islands previous, adjacent_islands
    end
  end

  def merge_previous_adjacent_islands(island)
    empty = IslandFactory.dummy_island
    adjacent_island = select_adjacent_to(island)
                      .reduce(empty) do |sum, adjacent|
      sum.merge! adjacent
    end
    @previous_islands.reject! { |previous| previous.adjacent? island }
    @previous_islands << adjacent_island unless adjacent_island.area.zero?
  end

  def new_islands_become_the_previous_islands(previous_island,
                                              adjacent_islands)
    previous_island.incorporate! adjacent_islands
    @next_islands << previous_island
  end

  def select_adjacent_to(island)
    @previous_islands.select { |previous| previous.adjacent? island }
  end
end
