pub struct List<Item>(Vec<Item>)
where
    Item: Clone;
impl<Item: Clone> List<Item> {
    pub fn from_vec(vec: Vec<Item>) -> Self {
        List(vec)
    }
    pub fn new() -> Self {
        List(Vec::new())
    }
    pub fn push(&mut self, item: Item) {
        self.0.push(item);
    }
    pub fn iter(&self) -> std::slice::Iter<Item> {
        self.0.iter()
    }
    pub fn map<R, F>(&self, f: F) -> List<R>
    where
        F: FnMut(&Item) -> R,
        R: Clone,
    {
        List(self.0.iter().map(f).collect())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn append(&mut self, other: &mut List<Item>) {
        self.append_vec(&mut other.0);
    }
    pub fn append_vec(&mut self, other: &mut Vec<Item>) {
        self.0.append(other);
    }
    pub fn get(&self, index: usize) -> Option<&Item> {
        self.0.get(index)
    }
    pub fn get_mut(&mut self, index:usize) -> Option<&mut Item> {
        self.0.get_mut(index)
    }
    pub unsafe fn get_unchecked(&self, index: usize) -> &Item {
        self.0.get_unchecked(index)
    }
    pub unsafe fn get_unchecked_mut(&mut self, index:usize) -> &mut Item {
        self.0.get_unchecked_mut(index)
    }
}
impl<Item: Clone + std::fmt::Display> List<Item> {
    pub fn join(&self, sep: &str) -> String {
        self.map(|item| item.to_string()).0.join(&sep)
    }
}
impl<Item: Clone> Clone for List<Item> {
    fn clone(&self) -> Self {
        List(self.0.iter().map(|item| item.clone()).collect())
    }
}
impl<Item: Clone + std::fmt::Display> std::fmt::Display for List<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str: Vec<String> = self.0.iter().map(|item| item.to_string()).collect();
        write!(f, "{}", str.join("\n"))
    }
}
impl<Item: Clone + PartialEq> PartialEq for List<Item> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            let self_item = self.get(i);
            let other_item = other.get(i);
            if self_item != other_item {
                return false;
            }
        }
        return true;
    }
}
impl<Item: Clone + Eq> Eq for List<Item> {}
impl <Item: Clone + PartialOrd> PartialOrd for List<Item> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.len() != other.len() {
            return None;
        }
        for i in 0..self.len() {
            let self_item = self.get(i);
            let other_item = other.get(i);
            if self_item != other_item {
                return None;
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
}
impl<Item: Clone + Ord> Ord for List<Item> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.len() != other.len() {
            return std::cmp::Ordering::Less;
        }
        for i in 0..self.len() {
            let self_item = self.get(i);
            let other_item = other.get(i);
            if self_item != other_item {
                return std::cmp::Ordering::Less;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}
impl<Item: Clone + std::hash::Hash> std::hash::Hash for List<Item> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for item in self.iter() {
            item.hash(state);
        }
    }
}
impl<Item: Clone> std::ops::Index<usize> for List<Item> {
    type Output = Item;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<Item: Clone> std::ops::IndexMut<usize> for List<Item> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl<Item: Clone + std::fmt::Debug> std::fmt::Debug for List<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str: Vec<String> = self.0.iter().map(|item| format!("{:?}", item)).collect();
        write!(f, "{}", str.join("\n"))
    }
}