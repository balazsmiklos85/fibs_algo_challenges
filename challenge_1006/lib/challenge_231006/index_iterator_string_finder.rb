# frozen_string_literal: true

require_relative 'substring_finder'

module Challenge231006
  # Finds the shortest substring of a given string that contains all the characters in a given set.
  # This implementation uses an index iterator approach, which is O(n * m), where n = target.length and m = characters.length.
  class IndexIteratorStringFinder < SubstringFinder
    # This method walks through the given string, starting with each character it walks through the string as long as it finds every character in the given set. Then it returns the shortest substring it found.
    def find_shortest_substring_containing_all_characters(target)
      result = nil
      0.upto(target.length - 1) do |start_index| # let's look from
        result_at_index = find_first_substring_containing_characters(target, start_index)
        result = shorter_of result_at_index, result
      end
      result
    end

    private

    def find_first_substring_containing_characters(target, start_index)
      characters = @characters
      result = []
      target[start_index..].each_char do |char| # walk through the substring
        characters = characters.sub(char, '') if characters.include? char # looking for this char -> cross it out
        result << char # collect characters
        return result.join '' if characters.empty? # return substring as soon as we have no more characters to look for
      end
      nil
    end

    def shorter_of(original, new_string)
      return original unless new_string.nil?

      return new_string if original.nil?

      return new_string if new_string.length < original.length

      original
    end
  end
end
