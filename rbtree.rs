//use core::to_str::*;

struct Color(bool);

priv static red: Color = Color(true);
priv static black: Color = Color(false);

priv struct RBTree<K, V> {
    color: Color,
    left: Option<~RBTree<K,V>>,
    right: Option<~RBTree<K,V>>,
    key: K,
    value: V
}

pub fn create<K:Ord + Eq, V>(k: K, v: V) -> RBTree<K, V> {
    RBTree {
        color: black,
        left: None,
        right: None,
        key: k,
        value: v
    }
}

impl<K: Ord + Eq, V> RBTree<K,V> {

    pub fn find(&self, k: K) -> Option<&'self V> {
        match *self {
            RBTree { key: ref key, value: ref v, _ } if k == *key => Some(v),
            RBTree { key: ref key, left: ref l, right: ref r, _ } => {
                if *key > k && l.is_some() {
                    l.get_ref().find(k)
                }
                else if *key < k && r.is_some() {
                    r.get_ref().find(k)
                }
                else {
                    None
                }
            }
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        if k < self.key {
            match self.left {
                Some(ref mut l) => l.insert(k, v),
                None => self.left = Some(~create(k, v))
            }
        }
        else if k > self.key {
            match self.right {
                Some(ref mut r) => r.insert(k, v),
                None => self.right = Some(~create(k, v))
            }
        }
    }

    fn height(&self) -> uint {
        match *self {
            RBTree { left: None, right: None, _ } => 1u,
            RBTree { left: ref l, right: ref r, _ } => 
            1u + max( l.map_default(0u, |t| t.height()),
                     r.map_default(0u, |t| t.height()) )
        }
    }

}

priv fn max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

impl<K, V: Copy + Add<V,V>> RBTree<K,V> {
    pub fn sum(&self) -> V {
        let mut val = copy self.value;
        if self.left.is_some() {
            val += self.left.get_ref().sum()
        }
        if self.right.is_some() {
            val += self.right.get_ref().sum()
        }      
        val
    }

}

#[test]
fn test_create() {
    let tree = create(5, "five");
    assert!(tree.key == 5 && tree.value == "five");
}

#[test]
fn test_insert() {
    let tree = @mut create(5, "five");
    tree.insert(6, "six");
    let mut l = tree.right.get_mut_ref();
    assert!( l.value == "six" && l.key == 6);
    assert!( l.right.is_none() );
}

#[test]
fn test_find() {
    let mut tree = @mut create(5, "five");
    tree.insert(4, "four");
    tree.insert(6, "six");
    match tree.find(6) {
        Some(r) => assert!("six" == *r),
        None => fail!(~"None is returned")
    }
}

#[test]
fn test_sum() {
    let tree = create("five", 5);
    assert!(5 == tree.sum());
}

#[test]
fn test_height() {
    let mut tree = @mut create(5, "five");
    assert!(tree.height() == 1u);
    tree.insert(4, "four");
    tree.insert(6, "six");
    assert!(tree.height() == 2u);
}

#[test]
fn test_balanced() {
    let mut tree = @mut create(4, "four");
    assert!(tree.height() == 1u);
    tree.insert(6, "six");
    tree.insert(5, "five");
    assert!(tree.height() == 2u);
}