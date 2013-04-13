//use core::to_str::*;

#[deriving(Clone, Eq)]
struct Color(bool);

priv static red: Color = Color(true);
priv static black: Color = Color(false);

priv struct RBTree<'self, K, V> {
    color: Color,
    left: Option<~RBTree<'self, K,V>>,
    right: Option<~RBTree<'self, K,V>>,
    key: K,
    value: V,
    parent: Option<&'self RBTree<'self, K,V>>
}

pub fn create<K:Ord + Eq, V>(k: K, v: V) -> RBTree<K, V> {
    create_with_color(k, v, black)
}

priv fn create_with_color<K:Ord + Eq, V>(k: K, v: V, c: Color) -> RBTree<K, V> {
    RBTree {
        color: c,
        left: None,
        right: None,
        key: k,
        value: v,
        parent: None
    }
}

impl<'self, K: Ord + Eq + Copy, V> RBTree<'self, K,V> {

    pub fn find(&self, k: K) -> Option<&'self V> {
        match *self {
            RBTree { key: ref key, value: ref v, _ } if k == *key => Some(v),
            RBTree { key: ref key, left: ref l, right: ref r, _ } => {
                if *key > k {
                    l.chain_ref(|t| t.find(k))
                }
                else {
                    r.chain_ref(|t| t.find(k))
                }
            }
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        if k < self.key {
            match self.left {
                Some(ref mut l) => l.insert(k, v),
                None => self.left = Some(self.make_child(k, v))
            }
        }
        else if k > self.key {
            match self.right {
                Some(ref mut r) => r.insert(k, v),
                None => self.right = Some(self.make_child(k, v))
            }
        }
    }

    priv fn height(&self) -> uint {
        match *self {
            RBTree { left: None, right: None, _ } => 1u,
            RBTree { left: ref l, right: ref r, _ } => 
            1u + max( l.map_default(0u, |t| t.height()),
                     r.map_default(0u, |t| t.height()) )
        }
    }

    priv fn grand(&self) -> Option<&'self RBTree<'self, K, V>> {
        self.parent.chain_ref(|p| p.parent)
    }

    priv fn make_child(&self, k: K, v: V) -> ~RBTree<'self, K, V> {
        ~RBTree {
            color: if self.color == black { red } else { black },
            left: None,
            right: None,
            key: k,
            value: v,
            parent: Some(self)
        }
    }
}

priv fn max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

impl<'self, K, V: Copy + Add<V,V>> RBTree<'self, K,V> {
    pub fn sum(&self) -> V {
        let mut val = copy self.value;
        self.left.map( |l| val += l.sum() );
        self.right.map( |r| val += r.sum() );
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
    let mut r = tree.right.get_mut_ref();
    assert!( r.value == "six" && r.key == 6);
    assert!( tree.left.is_none() );
}

#[test]
fn test_insert_color() {
   let tree = @mut create(5, "five");
    tree.insert(6, "six");
    let mut r = tree.right.get_mut_ref();
    assert!( r.color == red );
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
    let mut tree = @mut create("five", 5);
    tree.insert("four", 4);
    tree.insert("six", 6);
    assert!(15 == tree.sum());
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