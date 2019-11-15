use trie_rs::TrieBuilder;

fn main() {
    let mut builder = TrieBuilder::new();
    builder.push("怎样辨别真相和阴谋论？");

    let trie = builder.build();

    println!("{:?}", trie.exact_match("怎样辨别"));
    println!("{:?}", trie.match_partial("怎样别辨").is_none());
    println!("{:?}", trie.match_partial("怎样辨别").unwrap().exact_match("真相和阴谋论？"));
    println!("{:?}", trie.match_partial("怎样辨别").unwrap().match_partial("真相和阴谋论").unwrap().exact_match("？"));
    println!("{:?}", trie.match_partial("怎样辨别").unwrap().match_partial("真相和阴谋论？").unwrap().exact_match(""));
}