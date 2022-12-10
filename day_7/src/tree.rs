pub mod tree_impl {
    #[derive(Debug, Default)]
    pub struct Node <T> 
    where
        T: PartialEq + std::fmt::Display
    {
        pub idx: usize,
        pub val: T,
        pub size: Option<i32>,
        pub parent: Option<usize>,
        pub children: Vec<usize>,
    }

    impl<T> Node<T>
    where
        T: PartialEq + std::fmt::Display
    {
        fn new(idx: usize, val: T) -> Self {
            Self {
                idx,
                val,
                size: None,
                parent: None,
                children: vec![],
            }
        }
    }
    
    #[derive(Debug, Default)]
    pub struct Tree<T>
    where  
        T: PartialEq + std::fmt::Display
    {
        pub tree: Vec<Node<T>>,
    }

    impl<T> Tree<T>
    where
        T: PartialEq + std::fmt::Display
    {
        pub fn size(&self) -> usize 
        {
            return self.tree.len();
        }

        pub fn add(&mut self, value: T, parent: Option<usize>)
        {
            let idx = self.tree.len();
            self.tree.push(Node::new(idx, value));
            
            if parent.is_some()
            {
                self.tree[idx].parent = parent;
                self.tree[parent.unwrap()].children.push(idx);
            }
        }

        pub fn add_file(&mut self, value: T, parent: usize, size: i32)
        {
            let idx = self.tree.len();
            self.tree.push(Node::new(idx, value));
            
            self.tree[idx].parent = Some(parent);
            self.tree[idx].size = Some(size);
            self.tree[parent].children.push(idx);
        }

        pub fn find_child(&self, idx: usize, value: T) -> Option<usize> {
            let node = &self.tree[idx];
            
            for child in &node.children {
                if self.tree[*child].val == value {
                    return Some(self.tree[*child].idx);
                }
            }

            return None;
        }

        pub fn get_parent(&self, idx: usize) -> Option<usize> {
            return self.tree[idx].parent;
        }

        pub fn process_sizes(&mut self) {
            println!("Full Tree Size: {}", self.compute_size(0));
        }

        pub fn compute_size(&mut self, idx: usize) -> i32 {
            let node = &self.tree[idx];

            if node.size.is_some() {
                return node.size.unwrap();
            }

            let mut sum = 0;

            for child in node.children.clone() {
                sum += self.compute_size(child);
            }

            // self.tree[idx].size = Some(sum);
            return sum;
        }

        pub fn is_folder(&self, idx: usize) -> bool {
            return self.tree[idx].size.is_none();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::tree_impl::{Tree as Tree, Node as Node};

    #[test]
    fn test_empty() {
        let mut tree: Tree<String> = Tree::default();
        assert_eq!(tree.size(), 0);
    }

    #[test]
    fn test_add() {
        let mut tree: Tree<String> = Tree::default();
        
        tree.add("folder".to_string(), None);
        assert_eq!(tree.size(), 1);

        tree.add("folder_2".to_string(), None);
        tree.add("folder_3".to_string(), None);
        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn test_add_children() {
        let mut tree: Tree<String> = Tree::default();
        
        tree.add("folder".to_string(), None);
        tree.add("file".to_string(), Some(0));

        assert_eq!(tree.size(), 2);
        assert_eq!(tree.tree[0].children.len(), 1);
        assert_eq!(tree.tree[1].parent.unwrap(), 0);
    }
}