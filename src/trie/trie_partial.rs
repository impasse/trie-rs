use super::TriePartial;

impl<'a, Label: Ord + Clone> TriePartial<'a, Label> {

    pub fn match_partial<Arr: AsRef<[Label]>>(&self, query: Arr) -> Option<TriePartial<Label>> {
        let mut cur_node_num = self.cur_node_num;

        if query.as_ref().len() == 0 {
            return Some(TriePartial {
                trie: &self.trie,
                cur_node_num: cur_node_num,
            });
        }

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums = self.trie.children_node_nums(cur_node_num);
            let res = self.trie.bin_search_by_children_labels(chr, &children_node_nums[..]);

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    if i == query.as_ref().len() - 1 {
                        return Some(TriePartial {
                            trie: &self.trie,
                            cur_node_num: child_node_num,
                        });
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => return None,
            }
        }
        None
    }

    pub fn exact_match<Arr: AsRef<[Label]>>(&self, query: Arr) -> bool {
        let mut cur_node_num = self.cur_node_num;

        if query.as_ref().len() == 0 {
            return self.trie.is_terminal(cur_node_num);
        }

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums = self.trie.children_node_nums(cur_node_num);
            let res = self.trie.bin_search_by_children_labels(chr, &children_node_nums[..]);

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    if i == query.as_ref().len() - 1 && self.trie.is_terminal(child_node_num) {
                        return true;
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => return false,
            }
        }
        false
    }
}