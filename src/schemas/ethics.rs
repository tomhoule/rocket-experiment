pub use self::schema::ETHICS;
use std::str::FromStr;
use regex::Regex;
use inlinable_string::{InlinableString, StringExt};

#[derive(Serialize, Debug)]
pub struct ExpandedNode {
    pub depth: u8,
    pub path: InlinableString,
    pub node_type: NodeType,
    pub num: Option<u8>,
}

impl ExpandedNode {
    pub fn title(&self) -> &str {
        match self.node_type {
            NodeType::Scope(title) => title,
            NodeType::AnonymousFragment => "",
            ref other => other.segment_title(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Schema(Node);

impl Schema {
    pub fn root(self) -> Node {
        self.0
    }

    pub fn expand(&self) -> Vec<ExpandedNode> {
        let mut expanded = Vec::new();
        for node in self.0.children {
            node.expand("", 0, &mut expanded);
        }
        expanded
    }

    pub fn expand_part(&self, part_num: u8) -> Vec<ExpandedNode> {
        let mut expanded = Vec::new();
        if let Some(part) = self.0.children.get((part_num - 1) as usize) {
            part.expand("", 0, &mut expanded)
        }
        expanded
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Path(String);

impl Path {
    pub fn part(&self) -> Option<u8> {
        PATH_RE
            .captures(&self.0)
            .and_then(|cap| cap.name("part").and_then(|m| m.as_str().parse().ok()))
    }
}

lazy_static! {
    static ref PATH_RE: Regex = Regex::new(
        r"(?x)               # enable whitespace-insensitive mode
          ^pars/(?P<part>\d) # always starts with a numbered part
          (?:                # do not capture
            :([a-z_]+)       # a fragment type
            (?:
              /(\d+)
            )?               # an optional index
          )*                 # any number of times
          $                  # until the end of the string"
      ).expect("re is valid");
    static ref SEGMENT_RE: Regex = Regex::new(r"([a-z_]+)(?:/(\d+))?").unwrap();
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
}


#[derive(Clone, Debug, Serialize)]
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
            AnonymousFragment => "anon",
            Aliter => "aliter",
            Axioma => "axioma",
            Caput => "caput",
            Corollarium => "coroll",
            Definitio => "def",
            Demonstratio => "dem",
            Explicatio => "expl",
            Lemma => "lemma",
            Pars => "pars",
            Postulatum => "postul",
            Root => panic!("Root node should not appear in paths"),
            Scope(title) => title,
            Propositio => "p",
            Scholium => "scholium",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Node {
    node_type: NodeType,
    num: Option<u8>,
    children: &'static [Node],
}

impl Node {
    pub fn to_proto(&self) -> ::rpc::repository::EthicsSchema_Node {
        use protobuf::RepeatedField;
        use std::iter::FromIterator;
        use self::NodeType::*;
        use rpc::repository::EthicsSchema_NodeType::*;

        let mut node = ::rpc::repository::EthicsSchema_Node::new();
        let children = self.children.iter().map(|child| child.to_proto());
        let children = RepeatedField::from_iter(children);
        node.set_children(children);
        node.set_num(self.num.unwrap_or(0) as i32);
        let node_type = match self.node_type {
            AnonymousFragment => UNTITLED,
            Aliter => ALITER,
            Axioma => AXIOMA,
            Caput => CAPUT,
            Corollarium => COROLLARIUM,
            Definitio => DEFINITIO,
            Demonstratio => DEMONSTRATIO,
            Explicatio => EXPLICATIO,
            Lemma => LEMMA,
            Pars => PARS,
            Propositio => PROPOSITIO,
            Postulatum => POSTULATUM,
            Root => UNSPECIFIED,
            Scholium => SCHOLIUM,
            Scope(title) => {
                node.set_title(title.to_string());
                UNSPECIFIED
            }
        };
        node.set_node_type(node_type);
        node
    }

    fn expand<'a>(&'a self, prefix: &str, depth: u8, target: &mut Vec<ExpandedNode>) {
        use std::fmt::Write;
        let mut path = InlinableString::from(prefix);
        if depth > 0 {
            path.push(':')
        }
        path.push_str(self.node_type.segment_title());
        if let Some(num) = self.num {
            write!(path, "/{}", num).expect("String is writeable");
        }
        let expanded = ExpandedNode {
            depth,
            path: path.clone(),
            node_type: self.node_type.clone(),
            num: self.num.clone(),
        };
        target.push(expanded);
        for child in self.children {
            child.expand(&path, depth + 1, target);
        }
    }

    fn contains_path(&self, path: &str) -> bool {
        let mut node = Some(self);
        for segment in path.split(':') {
            let captures = SEGMENT_RE.captures(segment).expect("segment is parseable");
            let found_name: &str = &captures[1];
            let found_num: Option<u8> = captures
                .get(2)
                .map(|m| m.as_str())
                .and_then(|s| u8::from_str(s).ok());

            match node {
                Some(current) => for child in current.children.iter() {
                    // println!("segment: {:?}, path: {:?}", segment, path);
                    // println!(
                    //     "name {:?}, num {:?}  / name {:?}, num {:?}",
                    //     found_name,
                    //     found_num,
                    //     child.node_type.segment_title(),
                    //     child.num
                    // );
                    if found_name == child.node_type.segment_title() && found_num == child.num {
                        node = Some(child);
                        break;
                    }
                    node = None
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

    pub const ETHICS: Schema = Schema(Node {
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
                    node_type: Scope("defs"),
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
                    node_type: Scope("axiomata"),
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
                            num: Some(1),
                            children: &[],
                        },
                        Node {
                            node_type: Aliter,
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
                    node_type: Scope("appendix"),
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
                    node_type: Scope("praefatio"),
                    num: None,
                    children: &[],
                },
                Node {
                    node_type: Scope("defs"),
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
                    node_type: Scope("axiomata"),
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
                            children: &[
                                Node {
                                    node_type: Demonstratio,
                                    num: None,
                                    children: &[],
                                },
                            ],
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
                            node_type: Definitio,
                            num: None,
                            children: &[],
                        },
                        Node {
                            node_type: Axioma,
                            num: Some(3),
                            children: &[],
                        },
                    ],
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
                    node_type: Scope("postulata"),
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
                            node_type: Corollarium,
                            num: None,
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
                            children: &[
                                Node {
                                    node_type: Demonstratio,
                                    num: None,
                                    children: &[],
                                },
                            ],
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
                            children: &[
                                Node {
                                    node_type: Demonstratio,
                                    num: None,
                                    children: &[],
                                },
                            ],
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
                            children: &[
                                Node {
                                    node_type: Demonstratio,
                                    num: None,
                                    children: &[],
                                },
                            ],
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
                    node_type: Scope("praefatio"),
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
                    node_type: Scope("definitiones"),
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
                    node_type: Scope("postulata"),
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
                            children: &[
                                Node {
                                    node_type: Demonstratio,
                                    num: None,
                                    children: &[],
                                },
                            ],
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
                            node_type: Demonstratio,
                            num: None,
                            children: &[
                                Node {
                                    node_type: Corollarium,
                                    num: Some(1),
                                    children: &[],
                                },
                            ],
                        },
                        Node {
                            node_type: Corollarium,
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
                            node_type: Corollarium,
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
                            children: &[
                                Node {
                                    node_type: Demonstratio,
                                    num: None,
                                    children: &[],
                                },
                            ],
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
                            children: &[
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
                            num: Some(1),
                            children: &[
                                Node {
                                    node_type: Scholium,
                                    num: None,
                                    children: &[],
                                },
                            ],
                        },
                        Node {
                            node_type: Corollarium,
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
                    node_type: Scope("defs_aff"),
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
                    node_type: Scope("gen_def_aff"),
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
                    node_type: Scope("praefatio"),
                    num: None,
                    children: &[],
                },
                Node {
                    node_type: Scope("defs"),
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
                            children: &[
                                Node {
                                    node_type: Demonstratio,
                                    num: None,
                                    children: &[],
                                },
                            ],
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
                            children: &[
                                Node {
                                    node_type: Demonstratio,
                                    num: None,
                                    children: &[],
                                },
                            ],
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
                    node_type: Scope("appendix"),
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
                    node_type: Scope("praefatio"),
                    num: None,
                    children: &[],
                },
                Node {
                    node_type: Scope("axiomata"),
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
