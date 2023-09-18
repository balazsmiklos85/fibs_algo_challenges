#!/usr/bin/ruby

class OrgChartNode
    attr_reader :code, :directly_reports_to

    def initialize(code, manager)
      @code = code
      @directly_reports_to = manager
      @subordinates = []
      @directly_reports_to.add_subordinate self if has_manager?
    end

    def add_subordinate(subordinate)
      @subordinates.push subordinate
    end

    def has_manager?
      !@directly_reports_to.nil?
    end

    def to_s
      @code
    end
end

# O(n^2)
def lowest_common_manager_with_arrays(subordinate_1, subordinate_2)
  subordinate_1_managers = managerline_of subordinate_1
  subordinate_2_managers = managerline_of subordinate_2
  subordinate_1_managers.each do |manager|
    if subordinate_2_managers.include? manager
      return manager
    end
  end
end

# O(n) ~ O(n^2)
def lowest_common_manager_with_sets(subordinate_1, subordinate_2)
  subordinate_1_managers = managerline_of subordinate_1
  subordinate_2_managers = {}
  managerline_of(subordinate_2).each { |manager| subordinate_2_managers[manager.to_s] = true }
  subordinate_1_managers.each do |manager|
    if subordinate_2_managers.has_key? manager.to_s # O(1) ~ O(n)
      return manager
    end
  end
end

# O(n)
def lowest_common_manager_top_down(subordinate_1, subordinate_2)
  subordinate_1_managers = managerline_of subordinate_1
  subordinate_2_managers = managerline_of subordinate_2
  lowest_common = nil
  while !subordinate_1_managers.empty? && !subordinate_2_managers.empty?
    subordinate_1_manager = subordinate_1_managers.pop
    subordinate_2_manager = subordinate_2_managers.pop
    if subordinate_1_manager == subordinate_2_manager
      lowest_common = subordinate_1_manager
    else
      return lowest_common
    end
  end
  return lowest_common
end

def managerline_of(node)
    managerline = []
    while node.has_manager?
      node = node.directly_reports_to
      managerline.push node
    end
    return managerline
end

topManager = OrgChartNode.new 'A', nil
b = OrgChartNode.new 'B', topManager
c = OrgChartNode.new 'C', topManager
d = OrgChartNode.new 'D', b
e = OrgChartNode.new 'E', b
f = OrgChartNode.new 'F', c
g = OrgChartNode.new 'G', c
h = OrgChartNode.new 'H', d
i = OrgChartNode.new 'I', d

puts lowest_common_manager_with_arrays(e, i)
puts lowest_common_manager_with_sets(e, i)
puts lowest_common_manager_top_down(e, i)