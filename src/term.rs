pub type Name = String;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug,PartialEq)]
pub enum Term {
  Var(Name),
  Abstraction(Name, Rc<Term>),
  Application(Rc<Term>, Rc<Term>)
}

#[derive(Clone, Debug,PartialEq)]
pub enum TermOrDef {
    Definition(Name, Term),
    Term(Term)
}

pub struct Program(pub Vec<TermOrDef>);

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(name) => write!(f, "{}", name),
            Term::Abstraction(name, term) => write!(f, "(λ{}.{})", name, term),
            Term::Application(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}

impl fmt::Display for TermOrDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermOrDef::Definition(name, term) => write!(f, "{} := {}", name, term.to_string()),
            TermOrDef::Term(term) => write!(f, "{}", term.to_string())
        }
    }
}


impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.iter().fold(String::new(), |acc, x| acc+&x.to_string()+"\n");
        write!(f, "{}", s)
    }
}
