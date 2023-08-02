# frozen_string_literal: true

class BinaryTree
  def initialize(as_array)
    @as_array = as_array
  end

  def left_child(node)
    index = @as_array.find_index(node)
    @as_array[index * 2 + 1]
  end

  def right_child(node)
    index = @as_array.find_index(node)
    @as_array[index * 2 + 2]
  end

  def parent(node)
    index = @as_array.find_index(node)
    parent_index = (index - 1) / 2
    return nil if parent_index.negative?

    @as_array[parent_index]
  end

  def next_inorder(node)
    return nil if node.nil?

    first_right_parent(node) || leftmost(right_child(node))
  end

  def first_right_parent(node)
    return nil if node.nil?

    parent = parent(node)
    return nil if parent.nil?

    return parent unless right_child(parent) == node

    first_right_parent parent
  end

  def leftmost(node)
    return nil if node.nil?

    return node if left_child(node).nil?

    leftmost left_child(node)
  end
end

input = [1, 2, 3, 4, 5, nil, nil, 6]

tree = BinaryTree.new input
puts tree.next_inorder(5)
