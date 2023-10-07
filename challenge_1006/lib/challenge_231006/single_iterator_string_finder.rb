# frozen_string_literal: true

require_relative 'substring_finder'

module Challenge231006
  # Finds the shortest substring of a given string that contains all the characters in a given set.
  # This implementation traverses the string only once, moves the current substring along the string. O(n*m)
  class SingleIteratorStringFinder < SubstringFinder
    def find_shortest_substring_containing_all_characters(target)
      start! target
      shortest = nil
      while @current_end < target.length - 1
        break unless extend!

        shrink!
        shortest = current_substring if shorter_than? shortest
      end
      shortest.nil? ? 'N/A' : shortest
    end

    private

    def count_extra
      result = 0
      @contains.each do |key, value|
        extra = value - @characters.count(key)
        result += extra if extra.positive?
      end
      result
    end

    def current_substring
      result = @target[@current_start..@current_end]
      result.nil? ? 'N/A' : result.to_s
    end

    def extend!
      loop do
        @current_end += 1
        @contains.add! @target[@current_end]

        return true if match?
        return false if @current_end >= @target.length
      end
    end

    def match?
      @characters.each do |key, value|
        return false if value > @contains.count(key)
      end
      true
    end

    def shorter_than?(shortest)
      return true if shortest.nil?

      @current_end - @current_start + 1 < shortest.length
    end

    def shrink!
      until count_extra.zero?
        return unless shrinkable?

        @contains.remove! @target[@current_start]
        @current_start += 1
      end
    end

    def shrinkable?
      start_character = @target[@current_start]
      return true unless @characters.include? start_character

      return false unless @contains.include? start_character

      @contains.count(start_character) > @characters.count(start_character)
    end

    def start!(target)
      @current_start = 0
      @current_end = 0
      @target = target
      @contains = CharacterCounter.new @target[0]
    end
  end
end
