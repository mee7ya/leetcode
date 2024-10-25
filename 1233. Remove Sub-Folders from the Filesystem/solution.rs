struct Trie {
    val: String,
    subfolder_end: bool,
    children: Vec<Trie>,
}

impl Trie {
    pub fn new(val: String) -> Self {
        Trie {
            val,
            subfolder_end: false,
            children: Vec::<Trie>::new(),
        }
    }

    pub fn check(&self, val: &String) -> bool {
        let split = val.split("/");
        let mut node = self;
        for current in split {
            if let Some(child) = node.children.iter().find(|x| *x.val == *current) { node = child }
            else if node.subfolder_end { return false }
            else { return true }
        }
        true
    }

    pub fn insert(&mut self, val: String) {
        let split = val.split("/");
        let mut node = self;
        for current in split {
            if let Some(child) = node.children.iter_mut().position(|x| *x.val == *current) { node = &mut node.children[child] }
            else { 
                node.children.push(Trie::new(current.to_string()));
                node = node.children.last_mut().unwrap();
            }
        }
        node.subfolder_end = true;
    }
}

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        let mut root = Trie::new("$".to_string());
        let mut fits: Vec<bool> = Vec::with_capacity(folder.len());
        folder.sort_unstable();
        for item in folder.iter() {
            let can_fit = root.check(item);
            if can_fit { root.insert(item.clone()) }
            fits.push(can_fit)
        }
        folder.into_iter().zip(fits.into_iter()).filter(|(x, y)| *y).map(|(x, y)| x).collect()
    }
}
