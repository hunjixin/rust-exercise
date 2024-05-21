use std::marker::PhantomData;

struct Node<T> {
    val: T,
    right: SubTree<T>,
    left: SubTree<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            right: SubTree(None),
            left: SubTree(None),
        }
    }
}

struct TreeWalk<F, T> {
    callback: F,
    _data: PhantomData<T>,
}

impl<F: Fn(&T), T> TreeWalk<F, T> {
    fn walk(&self, tree: &SubTree<T>) {
        if let Some(val) = tree.as_ref() {
            (self.callback)(&val.val);
            self.walk(&val.left);
            self.walk(&val.right);
        }
    }
}

impl<F: FnMut(&mut Box<Node<T>>), T> TreeWalk<F, T> {
    fn walk_mut(&mut self, tree: &mut SubTree<T>) {
        if let Some(val) = tree.as_mut() {
            (self.callback)(val);
            self.walk_mut(&mut val.left);
            self.walk_mut(&mut val.right);
        }
    }
}

struct SubTree<T>(Option<Box<Node<T>>>);

impl<T> SubTree<T> {
    fn new(val: T) -> Self {
        SubTree(Some(Box::new(Node::new(val))))
    }

    fn empty() -> Self {
        SubTree(None)
    }

    fn set_val(&mut self, new_val: T) -> &mut Self {
        match self.0.as_mut() {
            Some(val) => {
                val.val = new_val;
            }
            None => self.0 = Some(Box::new(Node::new(new_val))),
        }
        self
    }

    fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn insert_left(&mut self, val: T) -> &mut SubTree<T> {
        let node = self.0.as_mut().expect("insert operate at some value");
        node.left = SubTree::new(val);
        &mut node.left
    }

    fn insert_right(&mut self, val: T) -> &mut SubTree<T> {
        let node = self.0.as_mut().expect("insert operate at some value");
        node.right = SubTree::new(val);
        &mut node.right
    }
}

impl<T> std::ops::Deref for SubTree<T> {
    type Target = Option<Box<Node<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for SubTree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut root = SubTree::new(1);

    let left_tree = root.insert_left(2);
    left_tree.insert_left(3);
    left_tree.insert_right(4);

    let right_tree = root.insert_right(5);
    right_tree.insert_left(6);
    right_tree.insert_right(7);

    {
        let display_fn = |v: &i32| {
            println!("value {}", v);
        };

        let walk = TreeWalk {
            callback: display_fn,
            _data: PhantomData::<i32>,
        };

        walk.walk(&root);
    }

    {
        let reverse_fn = |v: &mut Box<Node<i32>>| {
            let right_tmp = v.right.0.take();
            let left_tmp = v.left.0.take();

            if let Some(tmp_v) = right_tmp {
                v.left.0.replace(tmp_v);
            }

            if let Some(tmp_v) = left_tmp {
                v.right.0.replace(tmp_v);
            }
        };

        let mut walk = TreeWalk {
            callback: reverse_fn,
            _data: PhantomData::<i32>,
        };

        walk.walk_mut(&mut root);
    }

    {
        let display_fn = |v: &i32| {
            println!("value {}", v);
        };

        let walk = TreeWalk {
            callback: display_fn,
            _data: PhantomData::<i32>,
        };

        walk.walk(&root);
    }
}
