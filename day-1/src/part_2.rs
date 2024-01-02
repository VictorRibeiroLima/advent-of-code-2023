const A_INDEX: usize = 'a' as usize;

enum TrieResult {
    Word,
    Prefix,
    None,
}

#[derive(Clone)]
struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        let vec = vec![None; 26];
        TrieNode {
            children: vec,
            is_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        let mut trie = Trie {
            root: TrieNode::new(),
        };
        trie.insert("one");
        trie.insert("two");
        trie.insert("three");
        trie.insert("four");
        trie.insert("five");
        trie.insert("six");
        trie.insert("seven");
        trie.insert("eight");
        trie.insert("nine");
        return trie;
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            let index = ch as usize - A_INDEX;
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[index].as_mut().unwrap();
        }
        node.is_word = true;
    }

    fn search(&self, word: &str) -> TrieResult {
        let mut node = &self.root;
        for ch in word.chars() {
            let index = ch as usize - A_INDEX;
            if node.children[index].is_none() {
                return TrieResult::None;
            }
            node = node.children[index].as_ref().unwrap();
        }
        if node.is_word {
            return TrieResult::Word;
        }
        TrieResult::Prefix
    }
}

fn word_to_digit(word: &str) -> Option<char> {
    match word {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn process_line(line: &str, trie: &Trie) -> u32 {
    let mut first = '\n';
    let mut last = '\n';
    let mut word = "".to_string();
    for ch in line.chars() {
        if ch.is_numeric() {
            if first == '\n' {
                first = ch;
            } else {
                last = ch;
            }
            word = "".to_string();
        } else {
            word.push(ch);
            match trie.search(&word) {
                TrieResult::Word => {
                    if first == '\n' {
                        first = word_to_digit(&word).unwrap();
                    } else {
                        last = word_to_digit(&word).unwrap();
                    }
                    word = word.pop().unwrap().to_string();
                }
                TrieResult::None => {
                    word = re_search_word(word, &trie);
                }
                TrieResult::Prefix => {}
            }
        }
    }
    if last == '\n' {
        last = first;
    }
    let first = first.to_digit(10).unwrap();
    let last = last.to_digit(10).unwrap();
    let digit = first * 10 + last;
    return digit;
}

pub fn process(input: &str) -> u32 {
    let trie = Trie::new();
    let mut result = 0;
    for line in input.lines() {
        let digit = process_line(line, &trie);
        result += digit;
    }
    return result;
}

fn re_search_word(mut word: String, trie: &Trie) -> String {
    while word.len() > 0 {
        word.remove(0);
        match trie.search(&word) {
            TrieResult::None => {}
            _ => return word,
        }
    }
    return "".to_string();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("./inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn my_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 53340);
    }

    #[test]
    fn test_single_line() {
        let input = "4dsdllkqnpxglbseight";
        let result = process(input);
        assert_eq!(result, 48);
    }
}
