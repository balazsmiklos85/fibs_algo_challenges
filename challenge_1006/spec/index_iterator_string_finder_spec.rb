# frozen_string_literal: true

require 'challenge_231006/index_iterator_string_finder'

RSpec.describe Challenge231006::IndexIteratorStringFinder do
  describe '#find_shortest_substring_matching_containing_all_characters' do
    let(:finder) { Challenge231006::IndexIteratorStringFinder.new('$$abf') }

    it 'returns the shortest substring containing all characters' do
      expect(finder.find_shortest_substring_containing_all_characters('abcd$ef$axb$x$')).to eq('f$axb$')
    end
  end
end
