use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;
use louds_rs::LoudsNodeNum;

pub mod trie;
pub mod trie_builder;
pub mod trie_partial;

pub struct Trie<Label> {
    louds: Louds,
    /// (LoudsNodeNum - 2) -> TrieLabel
    trie_labels: Vec<TrieLabel<Label>>,
}

pub struct TriePartial<'a, Label> {
  trie: &'a Trie<Label>,
  cur_node_num: LoudsNodeNum,
}

pub struct TrieBuilder<Label> {
    naive_trie: NaiveTrie<Label>,
}

struct TrieLabel<Label> {
    label: Label,
    is_terminal: bool,
}
