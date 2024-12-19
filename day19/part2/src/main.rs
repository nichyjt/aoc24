use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            end: false,
        }
    }
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(Trie::new);
        }
        node.end = true;
    }

}

fn main() -> io::Result<()> {
    let res = File::open("../input.in");
    // Results need to be unwrapped. Prefer to use ? operator in the future
    let file = res.unwrap_or_else(|err| {
        eprintln!("Error loading file: {}", err);
        std::process::exit(1);
    });

    let reader = io::BufReader::new(file);
    let mut patterns= HashSet::<String>::new();
    let mut targets = Vec::<String>::new(); 
    
    let mut trie = Trie::new(); 
    
    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            let str = line?;
            let parts = str.split(", ");
            for p in parts {
                patterns.insert(p.to_string());
                trie.insert(p);
            }
        }else {
            let str = line?;
            if str.len() <= 0 {
                continue;
            }
            targets.push(str);
        }
    }

    let mut num_ways = 0;
    for target in targets {
        let mut memo = vec![None; target.len() + 1];
        let result = count_ways(&target, &trie, 0, &mut memo);
        num_ways += result;
    }
    println!("Result: {}", num_ways);

    Ok(())
}

fn count_ways(target: &str, trie: &Trie, start: usize, memo: &mut [Option<usize>]) -> usize {
    if start == target.len() {
        return 1; // valid 
    }

    if let Some(result) = memo[start] {
        return result; // check cache
    }

    let mut count = 0;
    let mut node = trie;
    // count ways from target[start] to target[target.len()-1]
    for i in start..target.len() {
        if let Some(child) = node.children.get(&target.chars().nth(i).unwrap()) {
            node = child;
            if node.end {
                // from 0..i we have found a match
                // so we count how many other ways [i+1..end] take 
                count += count_ways(target, trie, i + 1, memo);
            }
        } else {
            // no trie associated
            break;
        }
    }

    memo[start] = Some(count); // cache the result
    return count
}
