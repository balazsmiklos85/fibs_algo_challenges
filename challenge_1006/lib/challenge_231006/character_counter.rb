# frozen_string_literal: true

module Challenge231006
  class CharacterCounter
    def initialize(characters)
      @characters = {}
      characters.each_char do |character|
        if @characters.key? character
          @characters[character] += 1
        else
          @characters[character] = 1
        end
      end
    end

    def add!(character)
      @characters[character] = 0 unless @characters.key? character
      @characters[character] += 1
    end

    def count(character)
      return 0 unless @characters.key?(character)

      @characters[character]
    end

    def each(&block)
      @characters.each(&block)
    end

    def empty?
      @characters.each_value
                 .none?(&:positive?)
    end

    def include?(character)
      !character.nil? && @characters.key?(character) && !@characters[character].zero?
    end

    def remove!(character)
      return unless @characters.key? character

      @characters[character] -= 1
      @characters[character] = 0 if @characters[character].negative?
      self
    end

    def to_a
      result = []
      @characters.each_pair do |character, count|
        count.times do
          result << character
        end
      end
      result
    end
  end
end
