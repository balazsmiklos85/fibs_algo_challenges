# frozen_string_literal: true

require_relative 'substring_finder'

module Challenge231006
  # Finds the shortest substring of a given string that contains all the characters in a given set.
  # This implementation uses a brute force approach, which is O(n * m!), where n = target.length, m = characters.length.
  class RegexpStringFinder < SubstringFinder
    # This method generates all possible substrings of the given characters, turns them into non-greedy regular
    # expressions, and then scans the target string for them. It returns the shortest substring it found.
    def find_shortest_substring_containing_all_characters(target)
      @characters.split('') # "foo" => ["f", "o", "o"]
                 .permutation # ["f", "o", "o"] => [["f", "o", "o"], ["f", "o", "o"], ["o", "f", "o"], ["o", "o", "f"], ["o", "f", "o"], ["o", "o", "f"]]
                 .uniq # repeated patterns are not needed
                 .map { |characters| characters.map { |character| Regexp.escape character } } # ["f", "o", "o", "$"] => ["f", "o", "o", "\$"]
                 .map { |characters| characters.join '.*?' } # ["f", "o", "o"] => "f.*?o.*?o"
                 .map { |regex_string| Regexp.new regex_string } # "foo" => /foo/
                 .map { |regex| target.scan regex } # /fo.*o/ => ["foo", "fomo"]
                 .flatten
                 .min_by(&:length)
    end
  end
end
