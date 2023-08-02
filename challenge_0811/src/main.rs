pub struct BinaryTree {
    as_array: Vec<Option<i32>>,
}

impl BinaryTree {
    pub fn new(input: Vec<Option<i32>>) -> BinaryTree {
        BinaryTree { as_array: input }
    }

    pub fn left_child(&self, node: i32) -> Option<i32> {
        let index = self.as_array.iter().position(|&n| n == Some(node));
        if index.is_none() {
            return None;
        }
        let result = self.as_array.get(index.unwrap() * 2 + 1);
        if result.is_none() {
            return None;
        }
        *result.unwrap()
    }

    pub fn right_child(&self, node: Option<i32>) -> Option<i32> {
        if node.is_none() {
            return None;
        }
        let node = node.unwrap();
        let index = self.as_array.iter().position(|&n| n == Some(node));
        if index.is_none() {
            return None;
        }
        let result = self.as_array.get(index.unwrap() * 2 + 2);
        if result.is_none() {
            return None;
        }
        *result.unwrap()
    }

    pub fn parent(&self, node: i32) -> Option<i32> {
        let index = self.as_array.iter().position(|&n| n == Some(node));
        if index.is_none() || index.unwrap() == 0 {
            return None;
        }
        let parent_index = (index.unwrap() as i64 - 1) / 2;
        if parent_index < 0 || parent_index >= self.as_array.len() as i64 {
            return None;
        }

        self.as_array[parent_index as usize]
    }

    pub fn next_inorder(&self, node: i32) -> Option<i32> {
        let result = self.first_right_parent(Some(node));
        if result.is_some() {
            return result;
        }
        self.leftmost(self.right_child(Some(node)))
    }

    fn first_right_parent(&self, node: Option<i32>) -> Option<i32> {
        if node.is_none() {
            return None;
        }
        let node = node.unwrap();
        let parent = self.parent(node);
        if parent.is_none() {
            return None;
        }
        if self.right_child(parent) != Some(node) {
            return parent;
        }
        self.first_right_parent(parent)
    }

    fn leftmost(&self, node: Option<i32>) -> Option<i32> {
        if node.is_none() {
            return None;
        }
        let node = node.unwrap();
        let left = self.left_child(node);
        if left.is_none() {
            return Some(node);
        }
        self.leftmost(left)
    }
}

fn main() {
    let input = vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, None, Some(6)];

    let tree = BinaryTree::new(input);

    let next_node = tree.next_inorder(1);

    if next_node.is_some() {
        println!("{:?}", next_node.unwrap());
    } else {
        println!("null");
    }
}
