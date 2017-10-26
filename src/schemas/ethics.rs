pub use self::schema::ETHICA;
use std::str::FromStr;
use regex::Regex;

#[derive(Serialize, Debug)]
pub struct Schema(pub &'static [Node]);

#[derive(Debug, PartialEq)]
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
    static ref SEGMENT_RE: Regex = Regex::new(r"([a-z])+(?:\((\d)\))").unwrap();
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
        self.0.iter().any(|node| node.contains_path(path))
    }

    // fn all_paths(&self) -> Vec<Path> {
    //     unimplemented!();
    // }
}


#[derive(Serialize, Debug)]
#[serde(tag = "tag", content = "contents")]
pub enum Node {
    AnonymousFragment(NumberedFragment),
    Aliter,
    Appendix,
    Axioma(NumberedFragment),
    Caput(NumberedFragment),
    Corollarium(NumberedFragment),
    Definitio(NumberedFragment),
    Demonstratio,
    Explicatio,
    Scope(ScopeDescriptor),
    Lemma(NumberedFragment),
    Pars(NumberedFragment),
    Postulatum(NumberedFragment),
    Praefatio,
    Propositio(NumberedFragment),
    Scholium(NumberedFragment),
}

impl Node {
    pub fn to_protobuf(&self) -> ::rpc::repository::EthicsSchema_Node {
        use rpc::repository::*;
        use self::Node::*;
        use protobuf::RepeatedField;

        let mut node = EthicsSchema_Node::new();
        match *self {
            AnonymousFragment(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::UNTITLED);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Aliter => {
                node.set_node_type(EthicsSchema_NodeType::ALITER);
            }
            Appendix => {
                node.set_node_type(EthicsSchema_NodeType::APPENDIX);
            }
            Axioma(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::AXIOMA);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Caput(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::CAPUT);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Corollarium(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::COROLLARIUM);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Definitio(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::DEFINITIO);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Demonstratio => {
                node.set_node_type(EthicsSchema_NodeType::DEMONSTRATIO);
            }
            Explicatio => {
                node.set_node_type(EthicsSchema_NodeType::EXPLICATIO);
            }
            Lemma(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::LEMMA);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Pars(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::PARS);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Postulatum(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::POSTULATUM);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Praefatio => {
                node.set_node_type(EthicsSchema_NodeType::PRAEFATIO);
            }
            Propositio(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::PROPOSITIO);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Scholium(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_node_type(EthicsSchema_NodeType::SCHOLIUM);
                node.set_num(nf.num.unwrap_or(0) as i32);
                node.set_children(RepeatedField::from_vec(children));
            }
            Scope(ref nf) => {
                let children = nf.children.iter().map(|node| node.to_protobuf()).collect();
                node.set_title(nf.title.to_string());
                node.set_children(RepeatedField::from_vec(children));
            }
        }
        node
    }

    fn contains_path(&self, path: &Path) -> bool {
        let mut node = self;
        for segment in path.0.split(':') {
            let captures = SEGMENT_RE.captures(segment).unwrap();
            let (name, num) = node.to_segment();
            let found_name: &str = &captures[0];
            let found_num: Option<u8> = captures
                .get(1)
                .map(|m| m.as_str())
                .and_then(|s| u8::from_str(s).ok());

            if found_name != name || found_num != num {
                return false
            }

            unimplemented!();
        }
        true
    }

    fn to_segment(&self) -> (&'static str, Option<u8>) {
        use self::Node::*;
        match *self {
            AnonymousFragment(NumberedFragment { num: n, .. }) => ("anon", n),
            Aliter => ("aliter", None),
            Appendix => ("appendix", None),
            Axioma(NumberedFragment { num: n, .. }) => ("axioma", n),
            Caput(NumberedFragment { num: n, .. }) => ("caput", n),
            Corollarium(NumberedFragment { num: n, .. }) => ("corollarium", n),
            Definitio(NumberedFragment { num: n, .. }) => ("definitio", n),
            Demonstratio => ("demonstratio", None),
            Explicatio => ("explicatio", None),
            Scope(ScopeDescriptor { title: t, .. }) => (t, None),
            Lemma(NumberedFragment { num: n, .. }) => ("lemma", n),
            Postulatum(NumberedFragment { num: n, .. }) => ("postulatum", n),
            Praefatio => ("praefatio", None),
            Propositio(NumberedFragment { num: n, .. }) => ("propositio", n),
            Scholium(NumberedFragment { num: n, .. }) => ("scholium", n),
            Pars(NumberedFragment { num: n, .. }) => ("pars", n),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct NumberedFragment {
    // #[serde(default = "none")]
    num: Option<u8>,
    // #[serde(default = "empty_vec")]
    children: &'static [Node],
}

/// A nested fragment with a title
#[derive(Serialize, Debug)]
pub struct ScopeDescriptor {
    title: &'static str,
    children: &'static [Node],
}

// fn empty_vec<T>() -> &'static [T; 0] { &[] }
// fn none<T>() -> Option<T> { None }

macro_rules! nf {
    ($num:expr => $( $descendant:expr ),*) => {
        NumberedFragment {
            num: Some($num),
            children: &[ $( $descendant, )* ],
        }
    };
    ($num:expr) => {
        NumberedFragment {
            num: Some($num),
            children: &[],
        }
    };
    ($( $descendant:ty ),*) => {
        NumberedFragment {
            num: None,
            children: &[ $( $descendant, )* ],
        }
    };
}

pub mod schema {
    use super::*;
    use super::Node::*;

    pub const ETHICA: Schema = Schema(&[
        Pars(nf![
            1 =>
            Scope(ScopeDescriptor {
                title: "Definitiones",
                children: &[
                    Definitio(nf![1]),
                    Definitio(nf![2]),
                    Definitio(nf![3]),
                    Definitio(nf![4]),
                    Definitio(nf![5]),
                    Definitio(nf![6 => Explicatio]),
                    Definitio(nf![7]),
                    Definitio(nf![8 => Explicatio]),
                ],
            }),
            Scope(ScopeDescriptor {
                title: "Axiomata",
                children: &[
                    Axioma(nf![1]),
                    Axioma(nf![2]),
                    Axioma(nf![3]),
                    Axioma(nf![4]),
                    Axioma(nf![5]),
                    Axioma(nf![6]),
                    Axioma(nf![7]),
                ],
            }),
            Propositio(nf![1 => Demonstratio]),
            Propositio(nf![2 => Demonstratio]),
            Propositio(nf![3 => Demonstratio]),
            Propositio(nf![4 => Demonstratio]),
            Propositio(nf![5 => Demonstratio]),
            Propositio(nf![6 => Demonstratio, Corollarium(nf![]), Aliter]),
            Propositio(nf![7 => Demonstratio]),
            Propositio(nf![8 => Demonstratio, Scholium(nf![1]), Scholium(nf![2])]),
            Propositio(nf![9 => Demonstratio]),
            Propositio(nf![10 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![11 => Demonstratio, Aliter, Aliter, Scholium(nf![])]),
            Propositio(nf![12 => Demonstratio]),
            Propositio(nf![13 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![14 => Demonstratio, Corollarium(nf![1]), Corollarium(nf![2])]),
            Propositio(nf![15 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![16 =>
                                 Demonstratio,
                                 Corollarium(nf![1]),
                                 Corollarium(nf![2]),
                                 Corollarium(nf![3])]),
            Propositio(nf![17 => Demonstratio,
                                 Corollarium(nf![1]),
                                 Corollarium(nf![2]),
                                 Scholium(nf![])]),
            Propositio(nf![18 => Demonstratio]),
            Propositio(nf![19 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![20 => Demonstratio, Corollarium(nf![1]), Corollarium(nf![2])]),
            Propositio(nf![21 => Demonstratio]),
            Propositio(nf![22 => Demonstratio]),
            Propositio(nf![23 => Demonstratio]),
            Propositio(nf![24 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![25 => Demonstratio, Scholium(nf![]), Corollarium(nf![])]),
            Propositio(nf![26 => Demonstratio]),
            Propositio(nf![27 => Demonstratio]),
            Propositio(nf![28 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![29 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![30 => Demonstratio]),
            Propositio(nf![31 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![32 => Demonstratio, Corollarium(nf![1]), Corollarium(nf![2])]),
            Propositio(nf![33 => Demonstratio, Scholium(nf![1]), Scholium(nf![2])]),
            Propositio(nf![34 => Demonstratio]),
            Propositio(nf![35 => Demonstratio]),
            Propositio(nf![36 => Demonstratio]),
            Appendix
        ]),
        Pars(nf![
            2 =>
            Praefatio,
            Scope(ScopeDescriptor {
                title: "Definitiones",
                children: &[
                    Definitio(nf![1]),
                    Definitio(nf![2]),
                    Definitio(nf![3 => Explicatio]),
                    Definitio(nf![4 => Explicatio]),
                    Definitio(nf![5 => Explicatio]),
                    Definitio(nf![6]),
                    Definitio(nf![7]),
                ],
            }),
            Scope(ScopeDescriptor {
                title: "Axiomata",
                children: &[
                    Axioma(nf![1]),
                    Axioma(nf![2]),
                    Axioma(nf![3]),
                    Axioma(nf![4]),
                    Axioma(nf![5]),
                ],
            }),
            Propositio(nf![1 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![2 => Demonstratio]),
            Propositio(nf![3 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![4 => Demonstratio]),
            Propositio(nf![4 => Demonstratio]),
            Propositio(nf![6 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![7 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![8 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![9 => Demonstratio, Corollarium(nf![]), Demonstratio]),
            Propositio(nf![10 => Demonstratio,
                                 Scholium(nf![]),
                                 Corollarium(nf![]),
                                 Demonstratio,
                                 Scholium(nf![])]),
            Propositio(nf![11 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![12 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![13 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Axioma(nf![1]),
            Axioma(nf![2]),
            Lemma(nf![1 => Demonstratio]),
            Lemma(nf![2 => Demonstratio]),
            Lemma(nf![3 => Corollarium(nf![])]),
            Axioma(nf![1]),
            Axioma(nf![2 => Definitio(nf![])]),
            Axioma(nf![3]),
            Lemma(nf![4 => Demonstratio]),
            Lemma(nf![5 => Demonstratio]),
            Lemma(nf![6 => Demonstratio]),
            Lemma(nf![7 => Demonstratio, Scholium(nf![])]),
            Scope(ScopeDescriptor {
                title: "Postulata",
                children: &[
                    Postulatum(nf![1]),
                    Postulatum(nf![2]),
                    Postulatum(nf![3]),
                    Postulatum(nf![4]),
                    Postulatum(nf![5]),
                    Postulatum(nf![6]),
                ],
            }),
            Propositio(nf![14 => Demonstratio]),
            Propositio(nf![15 => Demonstratio]),
            Propositio(nf![16 => Demonstratio, Corollarium(nf![1]), Corollarium(nf![2])]),
            Propositio(nf![
                17 =>
                Demonstratio,
                Scholium(nf![]),
                Corollarium(nf![]),
                Demonstratio,
                Scholium(nf![])
            ]),
            Propositio(nf![18 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![19 => Demonstratio]),
            Propositio(nf![20 => Demonstratio]),
            Propositio(nf![21 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![22 => Demonstratio]),
            Propositio(nf![23 => Demonstratio]),
            Propositio(nf![24 => Demonstratio]),
            Propositio(nf![25 => Demonstratio]),
            Propositio(nf![26 => Demonstratio, Corollarium(nf![]), Demonstratio]),
            Propositio(nf![27 => Demonstratio]),
            Propositio(nf![28 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![29 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![30 => Demonstratio]),
            Propositio(nf![31 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![32 => Demonstratio]),
            Propositio(nf![33 => Demonstratio]),
            Propositio(nf![34 => Demonstratio]),
            Propositio(nf![35 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![36 => Demonstratio]),
            Propositio(nf![37 => Demonstratio]),
            Propositio(nf![38 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![39 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![40 => Demonstratio, Scholium(nf![1]), Scholium(nf![2])]),
            Propositio(nf![41 => Demonstratio]),
            Propositio(nf![42 => Demonstratio]),
            Propositio(nf![43 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![
                44 =>
                Demonstratio,
                Corollarium(nf![1]),
                Scholium(nf![]),
                Corollarium(nf![2]),
                Demonstratio
            ]),
            Propositio(nf![45 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![46 => Demonstratio]),
            Propositio(nf![47 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![48 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![
                49 =>
                Demonstratio,
                Corollarium(nf![]),
                Demonstratio,
                Scholium(nf![])
            ])
        ]),
        Pars(nf![
             3 =>
             Scope(ScopeDescriptor {
                 title: "Praefatio",
                 children: &[AnonymousFragment(nf![])],
             }),
             Scope(ScopeDescriptor {
                 title: "Definitiones",
                 children: &[
                     Definitio(nf![1]),
                     Definitio(nf![2]),
                     Definitio(nf![3]),
                 ],
             }),
             Scope(ScopeDescriptor {
                 title: "Postulata",
                 children: &[
                     Postulatum(nf![1]),
                     Postulatum(nf![2]),
                 ],
             }),
             Propositio(nf![1 => Demonstratio, Corollarium(nf![])]),
             Propositio(nf![2 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![3 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![4 => Demonstratio]),
             Propositio(nf![5 => Demonstratio]),
             Propositio(nf![6 => Demonstratio]),
             Propositio(nf![7 => Demonstratio]),
             Propositio(nf![8 => Demonstratio]),
             Propositio(nf![9 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![10 => Demonstratio]),
             Propositio(nf![11 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![12 => Demonstratio]),
             Propositio(nf![13 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
             Propositio(nf![14 => Demonstratio]),
             Propositio(nf![15 => Demonstratio, Corollarium(nf![]), Demonstratio, Scholium(nf![])]),
             Propositio(nf![16 => Demonstratio]),
             Propositio(nf![17 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![18 => Demonstratio, Scholium(nf![1]), Scholium(nf![2])]),
             Propositio(nf![19 => Demonstratio]),
             Propositio(nf![20 => Demonstratio]),
             Propositio(nf![21 => Demonstratio]),
             Propositio(nf![22 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![23 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![24 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![25 => Demonstratio]),
             Propositio(nf![26 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![
                 27 =>
                 Demonstratio,
                 Scholium(nf![]),
                 Corollarium(nf![1]),
                 Demonstratio,
                 Corollarium(nf![2]),
                 Demonstratio,
                 Corollarium(nf![3]),
                 Demonstratio,
                 Scholium(nf![])
             ]),
             Propositio(nf![28 => Demonstratio]),
             Propositio(nf![29 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![30 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![31 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
             Propositio(nf![32 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![33 => Demonstratio]),
             Propositio(nf![34 => Demonstratio]),
             Propositio(nf![35 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![
                 36 =>
                 Demonstratio,
                 Corollarium(nf![]),
                 Demonstratio,
                 Scholium(nf![])
             ]),
             Propositio(nf![37 => Demonstratio]),
             Propositio(nf![38 => Demonstratio]),
             Propositio(nf![40 =>
                 Demonstratio,
                 Scholium(nf![]),
                 Corollarium(nf![1]),
                 Corollarium(nf![2]),
                 Demonstratio,
                 Scholium(nf![])
             ]),
             Propositio(nf![41 =>
                 Demonstratio,
                 Scholium(nf![]),
                 Corollarium(nf![]),
                 Scholium(nf![])
             ]),
             Propositio(nf![42 => Demonstratio]),
             Propositio(nf![43 => Demonstratio]),
             Propositio(nf![44 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![45 => Demonstratio]),
             Propositio(nf![46 => Demonstratio]),
             Propositio(nf![47 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![48 => Demonstratio]),
             Propositio(nf![49 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![50 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![51 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![52 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![53 => Demonstratio, Corollarium(nf![])]),
             Propositio(nf![54 => Demonstratio]),
             Propositio(nf![55 =>
                 Demonstratio,
                 Corollarium(nf![]),
                 Scholium(nf![]),
                 Corollarium(nf![]),
                 Demonstratio,
                 Scholium(nf![])
             ]),
             Propositio(nf![56 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![57 => Demonstratio, Scholium(nf![])]),
             Propositio(nf![58 => Demonstratio]),
             Propositio(nf![59 => Demonstratio, Scholium(nf![])]),
             Scope(ScopeDescriptor {
                 title: "Affectuum definitiones",
                 children: &[
                     AnonymousFragment(nf![1 => Explicatio]),
                     AnonymousFragment(nf![2]),
                     AnonymousFragment(nf![3 => Explicatio]),
                     AnonymousFragment(nf![4 => Explicatio]),
                     AnonymousFragment(nf![5]),
                     AnonymousFragment(nf![6 => Explicatio]),
                     AnonymousFragment(nf![7 => Explicatio]),
                     AnonymousFragment(nf![8]),
                     AnonymousFragment(nf![9]),
                     AnonymousFragment(nf![10 => Explicatio]),
                     AnonymousFragment(nf![11 => Explicatio]),
                     AnonymousFragment(nf![12]),
                     AnonymousFragment(nf![13 => Explicatio]),
                     AnonymousFragment(nf![14]),
                     AnonymousFragment(nf![15 => Explicatio]),
                     AnonymousFragment(nf![16]),
                     AnonymousFragment(nf![17 => Explicatio]),
                     AnonymousFragment(nf![18 => Explicatio]),
                     AnonymousFragment(nf![19]),
                     AnonymousFragment(nf![20 => Explicatio]),
                     AnonymousFragment(nf![21]),
                     AnonymousFragment(nf![22 => Explicatio]),
                     AnonymousFragment(nf![23 => Explicatio]),
                     AnonymousFragment(nf![24 => Explicatio]),
                     AnonymousFragment(nf![25]),
                     AnonymousFragment(nf![26 => Explicatio]),
                     AnonymousFragment(nf![27 => Explicatio]),
                     AnonymousFragment(nf![28 => Explicatio]),
                     AnonymousFragment(nf![29 => Explicatio]),
                     AnonymousFragment(nf![30]),
                     AnonymousFragment(nf![31 => Explicatio]),
                     AnonymousFragment(nf![32 => Explicatio]),
                     AnonymousFragment(nf![33 => Explicatio]),
                     AnonymousFragment(nf![34]),
                     AnonymousFragment(nf![35]),
                     AnonymousFragment(nf![36]),
                     AnonymousFragment(nf![37]),
                     AnonymousFragment(nf![38 => Explicatio]),
                     AnonymousFragment(nf![39]),
                     AnonymousFragment(nf![40]),
                     AnonymousFragment(nf![41 => Explicatio]),
                     AnonymousFragment(nf![42 => Explicatio]),
                     AnonymousFragment(nf![43]),
                     AnonymousFragment(nf![44 => Explicatio]),
                     AnonymousFragment(nf![45]),
                     AnonymousFragment(nf![46]),
                     AnonymousFragment(nf![47]),
                     AnonymousFragment(nf![48 => Explicatio]),
                     AnonymousFragment(nf![])
                 ],
             }),
             Scope(ScopeDescriptor {
                 title: "Affectuus generalis definitio",
                 children: &[Explicatio],
             }),
             AnonymousFragment(nf![])
        ]),
        Pars(nf![
             4 =>
            Praefatio,
            Scope(ScopeDescriptor {
                title: "Praefatio",
                children: &[
                    Definitio(nf![1]),
                    Definitio(nf![2]),
                    Definitio(nf![3]),
                    Definitio(nf![4]),
                    Definitio(nf![5]),
                    Definitio(nf![6]),
                    Definitio(nf![7]),
                    Definitio(nf![8]),
                ],
            }),
            Axioma(nf![]),
            Propositio(nf![1 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![2 => Demonstratio]),
            Propositio(nf![3 => Demonstratio]),
            Propositio(nf![4 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![5 => Demonstratio]),
            Propositio(nf![6 => Demonstratio]),
            Propositio(nf![7 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![8 => Demonstratio]),
            Propositio(nf![9 => Demonstratio, Scholium(nf![]), Corollarium(nf![])]),
            Propositio(nf![10 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![11 => Demonstratio]),
            Propositio(nf![12 => Demonstratio, Corollarium(nf![]), Demonstratio]),
            Propositio(nf![13 => Demonstratio]),
            Propositio(nf![14 => Demonstratio]),
            Propositio(nf![15 => Demonstratio]),
            Propositio(nf![16 => Demonstratio]),
            Propositio(nf![17 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![18 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![19 => Demonstratio]),
            Propositio(nf![20 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![21 => Demonstratio]),
            Propositio(nf![22 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![23 => Demonstratio]),
            Propositio(nf![24 => Demonstratio]),
            Propositio(nf![25 => Demonstratio]),
            Propositio(nf![26 => Demonstratio]),
            Propositio(nf![27 => Demonstratio]),
            Propositio(nf![28 => Demonstratio]),
            Propositio(nf![29 => Demonstratio]),
            Propositio(nf![30 => Demonstratio]),
            Propositio(nf![31 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![32 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![33 => Demonstratio]),
            Propositio(nf![34 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![35 => Demonstratio,
                                 Corollarium(nf![1]),
                                 Corollarium(nf![2]),
                                 Scholium(nf![])]),
            Propositio(nf![36 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![37 => Demonstratio, Aliter, Scholium(nf![1]), Scholium(nf![2])]),
            Propositio(nf![38 => Demonstratio]),
            Propositio(nf![39 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![40 => Demonstratio]),
            Propositio(nf![41 => Demonstratio]),
            Propositio(nf![42 => Demonstratio]),
            Propositio(nf![43 => Demonstratio]),
            Propositio(nf![44 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![
                45 =>
                Demonstratio,
                Scholium(nf![]),
                Corollarium(nf![1]),
                Corollarium(nf![2]),
                Scholium(nf![])
            ]),
            Propositio(nf![46 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![47 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![48 => Demonstratio]),
            Propositio(nf![49 => Demonstratio]),
            Propositio(nf![50 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![51 => Demonstratio, Aliter, Scholium(nf![])]),
            Propositio(nf![52 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![53 => Demonstratio]),
            Propositio(nf![54 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![55 => Demonstratio]),
            Propositio(nf![56 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![57 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![58 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![59 => Demonstratio, Aliter, Scholium(nf![])]),
            Propositio(nf![60 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![61 => Demonstratio]),
            Propositio(nf![62 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![63 => Demonstratio, Corollarium(nf![]), Demonstratio, Scholium(nf![])]),
            Propositio(nf![64 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![65 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![66 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![67 => Demonstratio]),
            Propositio(nf![68 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![69 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![70 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![71 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![72 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![73 => Demonstratio, Scholium(nf![])]),
            Scope(ScopeDescriptor {
                title: "Appendix",
                children: &[
                    AnonymousFragment(nf![]),
                    Caput(nf![1]),
                    Caput(nf![2]),
                    Caput(nf![3]),
                    Caput(nf![4]),
                    Caput(nf![5]),
                    Caput(nf![6]),
                    Caput(nf![7]),
                    Caput(nf![8]),
                    Caput(nf![9]),
                    Caput(nf![10]),
                    Caput(nf![11]),
                    Caput(nf![12]),
                    Caput(nf![13]),
                    Caput(nf![14]),
                    Caput(nf![15]),
                    Caput(nf![16]),
                    Caput(nf![17]),
                    Caput(nf![18]),
                    Caput(nf![19]),
                    Caput(nf![20]),
                    Caput(nf![21]),
                    Caput(nf![22]),
                    Caput(nf![23]),
                    Caput(nf![24]),
                    Caput(nf![25]),
                    Caput(nf![26]),
                    Caput(nf![27]),
                    Caput(nf![28]),
                    Caput(nf![29]),
                    Caput(nf![30]),
                    Caput(nf![31]),
                    Caput(nf![32]),
                    Caput(nf![33])
                ],
            }),
            AnonymousFragment(nf![])
        ]),
        Pars(nf![
             5 =>
            Praefatio,
            Scope(ScopeDescriptor {
                title: "Axiomata",
                children: &[
                    AnonymousFragment(nf![1]),
                    AnonymousFragment(nf![2])
                ],
            }),
            Propositio(nf![1 => Demonstratio]),
            Propositio(nf![2 => Demonstratio]),
            Propositio(nf![3 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![4 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![5 => Demonstratio]),
            Propositio(nf![6 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![7 => Demonstratio]),
            Propositio(nf![8 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![9 => Demonstratio]),
            Propositio(nf![10 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![11 => Demonstratio]),
            Propositio(nf![12 => Demonstratio]),
            Propositio(nf![13 => Demonstratio]),
            Propositio(nf![14 => Demonstratio]),
            Propositio(nf![15 => Demonstratio]),
            Propositio(nf![16 => Demonstratio]),
            Propositio(nf![17 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![18 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![19 => Demonstratio]),
            Propositio(nf![20 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![21 => Demonstratio]),
            Propositio(nf![22 => Demonstratio]),
            Propositio(nf![23 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![24 => Demonstratio]),
            Propositio(nf![25 => Demonstratio]),
            Propositio(nf![26 => Demonstratio]),
            Propositio(nf![27 => Demonstratio]),
            Propositio(nf![28 => Demonstratio]),
            Propositio(nf![29 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![30 => Demonstratio]),
            Propositio(nf![31 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![32 => Demonstratio, Corollarium(nf![])]),
            Propositio(nf![33 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![34 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![35 => Demonstratio]),
            Propositio(nf![36 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![37 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![38 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![39 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![40 => Demonstratio, Corollarium(nf![]), Scholium(nf![])]),
            Propositio(nf![41 => Demonstratio, Scholium(nf![])]),
            Propositio(nf![42 => Demonstratio, Scholium(nf![])])
        ]),
    ]);
}
