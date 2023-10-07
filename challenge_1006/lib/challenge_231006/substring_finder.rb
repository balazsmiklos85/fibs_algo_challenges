# frozen_string_literal: true

require_relative 'character_counter'

# Smallest Substring Containing
#
# You're given two non-empty strings: a big string and a small string. Write a function that returns the smallest substring in the big string that contains all of the small string's characters.
# Note that:
# - The substring can contain other characters not found in the small string.
# - The characters in the substring don't have to be in the same order as they appear in the small string.
# - If the small string has duplicate characters, the substring has to contain those duplicate characters (it can also contain more, but not fewer).
# You can assume that there will only be one relevant smallest substring.
module Challenge231006
  # @type abstract
  class SubstringFinder
    def initialize(characters)
      @characters = CharacterCounter.new characters
    end

    # @type abstract
    def find_shortest_substring_containing_all_characters(target)
      raise NotImplementedError
    end
  end
end
