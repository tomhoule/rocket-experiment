pub use self::schema::ETHICA;
use std::str::FromStr;
use regex::Regex;

#[derive(Serialize, Debug)]
pub struct Schema(Node);

#[derive(Debug, Serialize, PartialEq)]
pub struct Path(String);

lazy_static! {
    static ref PATH_RE: Regex = Regex::new(
        r"(?x)           # enable whitespace-insensitive mode
          ^pars\(\d\)    # always starts with a numbered part
          (?:            # do not capture
            :([a-z]+)    # a fragment type
            (?:
              \((\d+)\)
            )?           # an optional index
          )+             # any number of times
          $              # until the end of the string"
      ).unwrap();
    static ref SEGMENT_RE: Regex = Regex::new(r"([a-z]+)(?:\((\d+)\))").unwrap();
}

impl FromStr for Path {
    type Err = ();

    fn from_str(input: &str) -> Result<Path, ()> {
        if PATH_RE.is_match(input) {
            Ok(Path(input.to_string()))
        } else {
            Err(())
        }
    }
}

impl Schema {
    pub fn contains_path(&self, path: &Path) -> bool {
        self.0.contains_path(&path.0)
    }

    // fn all_paths(&self) -> Vec<Path> {
    //     unimplemented!();
    // }
}


#[derive(Debug, Serialize)]
pub enum NodeType {
    AnonymousFragment,
    Aliter,
    Axioma,
    Caput,
    Corollarium,
    Definitio,
    Demonstratio,
    Explicatio,
    Lemma,
    Pars,
    Postulatum,
    Root,
    Scope(&'static str),
    Propositio,
    Scholium,
}

impl NodeType {
    fn segment_title(&self) -> &'static str {
        use self::NodeType::*;

        match *self {
            AnonymousFragment => "anonymous",
            Aliter => "aliter",
            Axioma => "axioma",
            Caput => "caput",
            Corollarium => "corollarium",
            Definitio => "definitio",
            Demonstratio => "demonstratio",
            Explicatio => "explicatio",
            Lemma => "lemma",
            Pars => "pars",
            Postulatum => "postulatum",
            Root => unreachable!(),
            Scope(title) => title,
            Propositio => "propositio",
            Scholium => "scholium",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Node {
    node_type: NodeType,
    num: Option<u8>,
    children: &'static [Node],
}

impl Node {
    // pub fn to_protobuf(&self) -> ::rpc::repository::EthicsSchema_Node {
    //     use rpc::repository::*;
    //     use protobuf::RepeatedField;

    //     let mut node = EthicsSchema_Node::new();
    //     if let Some(num) = self.num { node.set_num(num as i32) }
    //     let children = self.children.iter().map(|node| node.to_protobuf()).collect();
    //     node.set_children(RepeatedField::from_vec(children));
    //     node.set_title(self.node_type.segment_title().to_string());
    //     node
    // }

    fn contains_path(&self, path: &str) -> bool {
        let mut node = Some(self);
        for segment in path.split(':') {
            match node {
                Some(current) => {
                    for child in current.children.iter() {
                        let captures = SEGMENT_RE.captures(segment).unwrap();
                        let found_name: &str = &captures[1];
                        let found_num: Option<u8> = captures
                            .get(2)
                            .map(|m| m.as_str())
                            .and_then(|s| u8::from_str(s).ok());

                        println!("name {:?}, num {:?}  / name {:?}, num {:?}", found_name, found_num, child.node_type.segment_title(), child.num);
                        if found_name == child.node_type.segment_title() && found_num == child.num {
                            println!("{:?} {:?} == {:?} {:?}",  found_name, found_num, child.node_type.segment_title(), child.num);
                            node = Some(child);
                            break
                        }
                        node = None
                    }
                },
                None => return false,
            }
        }
        !node.is_none()
    }
}

pub mod schema {
    use super::*;
    use super::NodeType::*;

    pub const ETHICA: Schema = Schema(Node {
        node_type: Root,
        num: None,
        children: PARTS,
    });

    const PARTS: &'static [Node] = &[
    Node {
        node_type: Pars,
        num: Some(1),
        children: &[
            Node {
                node_type: Scope("Definitiones"),
                num: None,
                children: &[
                    Node {
                        node_type: Definitio,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(3),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(4),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(5),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(6),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(7),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(8),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                ],
            },
            Node {
                node_type: Scope("Axiomata"),
                num: None,
                children: &[
                    Node {
                        node_type: Axioma,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(3),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(4),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(5),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(6),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(7),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(1),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(2),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(3),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(4),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(5),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(6),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Aliter,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(7),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(8),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(9),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(10),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(11),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Aliter,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Aliter,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(12),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(13),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(14),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(15),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(16),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(3),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(17),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(18),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(19),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(20),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(21),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(22),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(23),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(24),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(25),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(26),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(27),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(28),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(29),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(30),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(31),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(32),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(33),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(34),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(35),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(36),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Scope("Appendix"),
                num: None,
                children: &[],
            },
        ],
    },
    Node {
        node_type: Pars,
        num: Some(2),
        children: &[
            Node {
                node_type: Scope("Praefatio"),
                num: None,
                children: &[],
            },
            Node {
                node_type: Scope("Definitiones"),
                num: None,
                children: &[
                    Node {
                        node_type: Definitio,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(3),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(4),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(5),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(6),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(7),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Scope("Axiomata"),
                num: None,
                children: &[
                    Node {
                        node_type: Axioma,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(3),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(4),
                        children: &[],
                    },
                    Node {
                        node_type: Axioma,
                        num: Some(5),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(1),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(2),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(3),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(4),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(4),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(6),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(7),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(8),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(9),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(10),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(11),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(12),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(13),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Axioma,
                num: Some(1),
                children: &[],
            },
            Node {
                node_type: Axioma,
                num: Some(2),
                children: &[],
            },
            Node {
                node_type: Lemma,
                num: Some(1),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Lemma,
                num: Some(2),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Lemma,
                num: Some(3),
                children: &[
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Axioma,
                num: Some(1),
                children: &[],
            },
            Node {
                node_type: Axioma,
                num: Some(2),
                children: &[
                    Node {
                        node_type: Definitio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Axioma,
                num: Some(3),
                children: &[],
            },
            Node {
                node_type: Lemma,
                num: Some(4),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Lemma,
                num: Some(5),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Lemma,
                num: Some(6),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Lemma,
                num: Some(7),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Scope("Postulata"),
                num: None,
                children: &[
                    Node {
                        node_type: Postulatum,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Postulatum,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Postulatum,
                        num: Some(3),
                        children: &[],
                    },
                    Node {
                        node_type: Postulatum,
                        num: Some(4),
                        children: &[],
                    },
                    Node {
                        node_type: Postulatum,
                        num: Some(5),
                        children: &[],
                    },
                    Node {
                        node_type: Postulatum,
                        num: Some(6),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(14),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(15),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(16),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(17),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(18),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(19),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(20),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(21),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(22),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(23),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(24),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(25),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(26),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(27),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(28),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(29),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(30),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(31),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(32),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(33),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(34),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(35),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(36),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(37),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(38),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(39),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(40),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(41),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(42),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(43),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(44),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(45),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(46),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(47),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(48),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(49),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
        ],
    },
    Node {
        node_type: Pars,
        num: Some(3),
        children: &[
            Node {
                node_type: Scope("Praefatio"),
                num: None,
                children: &[
                    Node {
                        node_type: AnonymousFragment,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Scope("Definitiones"),
                num: None,
                children: &[
                    Node {
                        node_type: Definitio,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(3),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Scope("Postulata"),
                num: None,
                children: &[
                    Node {
                        node_type: Postulatum,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Postulatum,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(1),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(2),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(3),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(4),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(5),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(6),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(7),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(8),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(9),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(10),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(11),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(12),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(13),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(14),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(15),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(16),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(17),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(18),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(19),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(20),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(21),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(22),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(23),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(24),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(25),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(26),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(27),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(3),
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(28),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(29),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(30),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(31),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(32),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(33),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(34),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(35),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(36),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(37),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(38),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(40),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(41),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(42),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(43),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(44),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(45),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(46),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(47),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(48),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(49),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(50),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(51),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(52),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(53),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(54),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(55),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(56),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(57),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(58),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(59),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Scope("Affectuum definitiones"),
                num: None,
                children: &[
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(1),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(3),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(4),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(5),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(6),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(7),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(8),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(9),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(10),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(11),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(12),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(13),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(14),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(15),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(16),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(17),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(18),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(19),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(20),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(21),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(22),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(23),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(24),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(25),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(26),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(27),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(28),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(29),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(30),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(31),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(32),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(33),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(34),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(35),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(36),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(37),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(38),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(39),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(40),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(41),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(42),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(43),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(44),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(45),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(46),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(47),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(48),
                        children: &[
                            Node {
                                node_type: Explicatio,
                                num: None,
                                children: &[],
                            },
                        ],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Scope("Affectuus generalis definitio"),
                num: None,
                children: &[
                    Node {
                        node_type: Explicatio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: AnonymousFragment,
                num: None,
                children: &[],
            },
        ],
    },
    Node {
        node_type: Pars,
        num: Some(4),
        children: &[
            Node {
                node_type: Scope("Praefatio"),
                num: None,
                children: &[],
            },
            Node {
                node_type: Scope("Praefatio"),
                num: None,
                children: &[
                    Node {
                        node_type: Definitio,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(3),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(4),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(5),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(6),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(7),
                        children: &[],
                    },
                    Node {
                        node_type: Definitio,
                        num: Some(8),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Axioma,
                num: None,
                children: &[],
            },
            Node {
                node_type: Propositio,
                num: Some(1),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(2),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(3),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(4),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(5),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(6),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(7),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(8),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(9),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(10),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(11),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(12),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(13),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(14),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(15),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(16),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(17),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(18),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(19),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(20),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(21),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(22),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(23),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(24),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(25),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(26),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(27),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(28),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(29),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(30),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(31),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(32),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(33),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(34),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(35),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(36),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(37),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Aliter,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(38),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(39),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(40),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(41),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(42),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(43),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(44),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(45),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(46),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(47),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(48),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(49),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(50),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(51),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Aliter,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(52),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(53),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(54),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(55),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(56),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(57),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(58),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(59),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Aliter,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(60),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(61),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(62),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(63),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(64),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(65),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(66),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(67),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(68),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(69),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(70),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(71),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(72),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(73),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Scope("Appendix"),
                num: None,
                children: &[
                    Node {
                        node_type: AnonymousFragment,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(2),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(3),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(4),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(5),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(6),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(7),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(8),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(9),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(10),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(11),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(12),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(13),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(14),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(15),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(16),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(17),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(18),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(19),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(20),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(21),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(22),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(23),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(24),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(25),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(26),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(27),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(28),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(29),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(30),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(31),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(32),
                        children: &[],
                    },
                    Node {
                        node_type: Caput,
                        num: Some(33),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: AnonymousFragment,
                num: None,
                children: &[],
            },
        ],
    },
    Node {
        node_type: Pars,
        num: Some(5),
        children: &[
            Node {
                node_type: Scope("Praefatio"),
                num: None,
                children: &[],
            },
            Node {
                node_type: Scope("Axiomata"),
                num: None,
                children: &[
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(1),
                        children: &[],
                    },
                    Node {
                        node_type: AnonymousFragment,
                        num: Some(2),
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(1),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(2),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(3),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(4),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(5),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(6),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(7),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(8),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(9),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(10),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(11),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(12),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(13),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(14),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(15),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(16),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(17),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(18),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(19),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(20),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(21),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(22),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(23),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(24),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(25),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(26),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(27),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(28),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(29),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(30),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(31),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(32),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(33),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(34),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(35),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(36),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(37),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(38),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(39),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(40),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Corollarium,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(41),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
            Node {
                node_type: Propositio,
                num: Some(42),
                children: &[
                    Node {
                        node_type: Demonstratio,
                        num: None,
                        children: &[],
                    },
                    Node {
                        node_type: Scholium,
                        num: None,
                        children: &[],
                    },
                ],
            },
        ],
    },
    ];
}
