use std::vec;
use std::fs;
use yaml_rust::{YamlLoader, YamlEmitter};

extern crate yaml_rust;

struct Cls {
    name: String
}

///
/// `parttern`: Used in the preprocessing link for reading files
pub struct Memory {
    cls_vec:  Vec<Cls>,
    parttern: Parttern, 
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum TextPiece {
    Word(String), //for example: cls, var, if
    Block(i32),
    Keyword(String), //for example: WORD
    Root,
}
#[derive(PartialEq, Debug)]
enum CanExpress {
    True,
    Block(i32),
    False,
}
impl TextPiece {
    fn can_express(&self, piece: &str) -> CanExpress {
        match self {
            Self::Word(w) => if w == piece {
                CanExpress::True
            } else {
                CanExpress::False
            },
            Self::Block(i) => CanExpress::Block(*i),
            //TODO: Now Keyword only support "WORD"
            Self::Keyword(s) => if Memory::judge_is_word(s){
                CanExpress::True
            } else {
                CanExpress::False
            },
            Self::Root => {
                panic!("So why a root?")
            }
        }
    }
}
#[derive(Debug)]
struct PartternEdge {
    to: usize,
    next: Option<usize>,
}
#[derive(Debug)]
struct PartternNode {
    pub text_piece: TextPiece,
    pub first_edge: Option<usize>,
    pub sentence_if_end: Option<u32>,
}
#[derive(Debug)]
struct Parttern {
    edges: Vec<PartternEdge>,
    nodes: Vec<PartternNode>,
    sentence_cnt: u32,
}
impl Parttern {
    fn new() -> Parttern {
        let yaml = fs
            ::read_to_string("src/config/parttern.yaml")
            .expect("Cannot read the file src/config/parttern.yaml");
        let yaml = YamlLoader
            ::load_from_str(&yaml)
            .unwrap();
        let yaml = &yaml[0];

        let root = PartternNode{
            text_piece: TextPiece::Root,
            first_edge: None,
            sentence_if_end: None,
        };
        
        let mut parttern = Parttern{
            edges: Vec::new(),
            nodes: vec![root],
            sentence_cnt: 0,
        };
        for sentence in yaml["sentense"].as_vec().unwrap() {
            let words = Memory
                ::split_sentence(sentence.as_str().unwrap());
            #[cfg(test)]
            println!("{:?}", words);
            if words == None {
                continue;
            }
            let mut sentence = Vec::new();
            for word in words.unwrap() {
                sentence.push(match &word[..] {
                    "WORD" => {
                        TextPiece::Keyword(String::from("WORD"))
                    },
                    _ => TextPiece::Word(word)
                });
            }

            parttern.add_sentences(sentence);
        };
        #[cfg(test)]
        println!("{:?}", parttern);
        parttern
    }

    fn add_sentences(&mut self, text_pieces: Vec<TextPiece>) {
        let mut i_node = 0;
        'text_pieces:
        for i in 0..text_pieces.len() {
            let text_piece = &text_pieces[i];
            let next = self.nodes[i_node].first_edge;
            let mut a_next = next;
            while let Some(mut next) = a_next {
                if &self.nodes[self.edges[next].to].text_piece == text_piece {
                    i_node = self.edges[next].to;
                    continue 'text_pieces;
                }
                a_next = self.edges[next].next;
            }
            let sentence_if_end =
                if i == text_pieces.len() - 1 {
                    self.sentence_cnt += 1;
                    Some(self.sentence_cnt - 1)
                } else {
                    None
            };
            self.nodes.push(PartternNode{
                text_piece: (*text_piece).clone(), 
                first_edge: None, 
                sentence_if_end
            });

            self.edges.push(PartternEdge { to: self.nodes.len() - 1, next });
            self.nodes[i_node].first_edge = Some(self.edges.len() - 1);
            i_node = self.nodes.len() - 1;
        }
    }

    fn _add_edge_if_not_exist(&mut self, from_node: usize, to_node: usize) {
        let next = self.nodes[from_node].first_edge;
        let mut a_next = next;
        while let Some(next) = a_next {
            if self.edges[next].to == to_node {
                return;
            }
            a_next = self.edges[next].next;
        }
        self.edges.push(PartternEdge { to: to_node, next })
    }
}
impl Memory {
    pub fn new() -> Memory {
        Memory { cls_vec: Vec::new(), parttern: Parttern::new()  }
    }
    pub fn read(&mut self, content: String) {
        let parttern = &self.parttern;
        let mut pieces: Vec<String> = Vec::new();

        for sentence in content.lines() {
            let pieces_tline = Self::split_sentence(sentence);
            if pieces_tline == None {
                continue;
            }
            pieces.append(&mut pieces_tline.unwrap());
        }
        println!("{:?}", pieces);
        self.read_piece(&pieces, 0, 0);
    }

    /// Deal with a single piece 
    /// 
    /// # Params:
    /// - `pieces`: The whole content of file, devided to pieces
    /// - `node_id`: The last Pattern Node readed
    /// - `next_node_id`: This Pattern Node readed
    fn read_piece(
        &mut self, 
        pieces: &Vec<String>, 
        finished_cnt: usize,
        node_id: usize,
    ) -> bool {
        if pieces.len() == finished_cnt {
            return true;
        }
        let parttern = &self.parttern;
        let mut edge = parttern.nodes[node_id].first_edge;
        let piece = &pieces[finished_cnt];
        println!("edges: {:?}", edge);
        #[cfg(test)]
        println!("pattern Node: {:?}", &self.parttern.nodes[0]);
        while let Some(u_edge) = edge {
            let mut next_node_id = self.parttern.edges[u_edge].to;
            let pattern_node: &PartternNode = &self.parttern.nodes[next_node_id];
            println!(
                "{:?}, {:?} {:?}", 
                pattern_node, 
                pattern_node.text_piece.can_express(&piece),
                piece
            );
            if pattern_node.text_piece.can_express(&piece) == CanExpress::True {

                println!("{:?}", pattern_node.sentence_if_end);
                //TODO: Now only support `class WORD` sentence
                if let Some(sentence_id) = pattern_node.sentence_if_end {
                    if sentence_id >= 2 {
                        eprintln!("Fix the wrong Sentence ID");
                    }
                    self.cls_vec.push(Cls { name: piece.to_owned() });
                    next_node_id = 0;
                }

                match self.read_piece(
                    &pieces,
                    finished_cnt + 1,
                    next_node_id,
                ) {
                    true => {return true},
                    false => {        
                        println!("poped");
                        //TODO: Now only support `class WORD` sentence
                        self.cls_vec.pop();
                    }
                }
            }
            edge = self.parttern.edges[u_edge].next;
        }
        false
    }

    /// Describe whats in Memory
    /// 
    /// Now All of them are classes
    pub fn get_desc(&self) -> String {
        let mut s = String::from("");
        for cls in &self.cls_vec {
            s.push_str(&format!("\n{}", cls.name));
        }
        format!("[classes]{}", s)
    }


    fn split_sentence(s: &str) -> Option<Vec<String>> {
        let mut v: Vec<String> = vec![];
        if s.len() == 0 {
            return None;
        }
        let mut last_word = String::from(s.chars().nth(0).unwrap());
        let mut last_word_type = match s.chars().nth(0).unwrap() {
            'a'..='z' | 'A'..='Z' => "word",
            '0'..='9' => "number",
            ' ' => "seperator",
            _ => "puctuation",
        };
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                continue;
            }
            let word_type = match c {
                'a'..='z' | 'A'..='Z' => "word",
                '0'..='9' => "number",
                ' ' => "seperator",
                _ => "puctuation",
            };
            match (last_word_type, word_type) {
                ("seperator", "seperator") => continue,
                (_, "seperator") => {
                    v.push(last_word.clone());
                    last_word = String::from("");
                    last_word_type = "seperator";
                },
                ("number", "number") |
                ("word", "word") | ("word", "number")=> {
                    last_word.push(c);
                },
                _ => {
                    if last_word != "" {
                        v.push(last_word.clone());
                    }
                    last_word.push(c);
                    last_word_type = word_type;
                }
            }
        };
        if last_word_type != "sepearator" && last_word != ""{
            v.push(last_word.clone());
        }
        Some(v)
    }

    fn judge_is_word(s: &str) -> bool {
        match Self::split_sentence(s) {
            Some(v) if v.len() == 1 => {
                match v[0].as_bytes()[0] as char {
                    'a'..='z' | 'A'..='Z' => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }
}