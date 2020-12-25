use crate::term::{Term, TermOrDef, Program};
use crate::name_gen::{NameGen};

use std::collections::HashMap;

fn is_free(term: &Term, var: &str) -> bool {
    match term {
        Term::Var(var2) => var != var2,
        Term::Abstraction(name, term) => (name != var) && is_free(term, var),
        Term::Application(t1, t2) => is_free(t1, var) && is_free(t2, var)
    }
}

fn perform_lookups(term: &Term, hm: &HashMap<String, Term>) -> Term {
    match term {
        Term::Var(name) => {
            if hm.contains_key(name) {
                hm.get(name).unwrap().clone()
            } else {
                term.clone()
            }
        },
        Term::Abstraction(param, body) => {
            Term::Abstraction(param.to_string(), Box::new(perform_lookups(body, hm)))
        },
        Term::Application(t1, t2) => Term::Application(Box::new(perform_lookups(t1, hm)), Box::new(perform_lookups(t2, hm)))
    }
}

// alpha reduction
fn rewrite(term: &Term, var: &str, new_var: &str) -> Term {
    match term {
        Term::Var(name) => {
            if name == var {
                Term::Var(new_var.to_string())
            } else {
                term.clone()
            }
        },
        Term::Abstraction(param, body) => {
            if param == var {
                Term::Abstraction(new_var.to_string(), Box::new(rewrite(body, var, new_var)))
            } else {
                Term::Abstraction(param.to_string(), Box::new(rewrite(body, var, new_var)))
            }
        },
        Term::Application(t1, t2) => Term::Application(Box::new(rewrite(&(**t1), var, new_var)), Box::new(rewrite(&(**t2), var, new_var)))
    }
}

fn replace(term: &Term, var: &str, substitution: &Term, ng: &mut NameGen, hm: &HashMap<String, Term>) -> Term {
    match term {
        Term::Var(name) => {
            if name == var {
                substitution.clone()
            } else {
                term.clone()
            }
        },
        Term::Abstraction(param, body) => {
            if param == var {
                term.clone()
            } else if is_free(substitution, param) {
                Term::Abstraction(param.to_string(), Box::new(replace(body, var, substitution, ng, hm)))
            } else {
                let mut rewritten_param = param.to_string();
                while !is_free(substitution, &rewritten_param) {
                    rewritten_param = ng.next().unwrap().to_string();
                }
                Term::Abstraction(param.to_string(), Box::new(replace(body, var, &rewrite(substitution, param, &rewritten_param), ng, hm)))
            }
        },
        Term::Application(t1, t2) => reduce_term(&Term::Application(Box::new(replace(&(**t1), var, substitution, ng, hm)), Box::new(replace(&(**t2), var, substitution, ng, hm))), ng, hm)
    }
}

pub fn reduce_term(term: &Term, ng: &mut NameGen, hm: &HashMap<String, Term>) -> Term {
    let term = perform_lookups(term, hm);

    match term {
        Term::Application(t1, t2) => match *t1 {
            Term::Abstraction(param, body) => {
                replace(&Box::new(body), &param, &Box::new(reduce_term(&t2, ng, hm)), ng, hm)
            },
            _ => {
                let t1p = reduce_term(&t1, ng, hm);
                let t2p = reduce_term(&t2, ng, hm);
                if &t1p != &*t1 || &t2p != &*t2 {
                    reduce_term(&Term::Application(Box::new(t1p), Box::new(t2p)), ng, hm)
                } else {
                    Term::Application(Box::new(t1p), Box::new(t2p))
                }
            }
        },
        _ => term.clone()
    }
}

pub fn reduce(program: &Program, ng: &mut NameGen) -> Program {
    let mut definitions: HashMap<String, Term> = HashMap::new();

    Program(program.0.iter().fold(Vec::new(), |mut v, term| {
        match term {
            TermOrDef::Definition(name, term) => {
                definitions.insert(name.to_string(), reduce_term(&term, ng, &definitions));
                v
            }
            TermOrDef::Term(t) => {
                v.push(TermOrDef::Term(reduce_term(&t, ng, &definitions)));
                v
            }
        }
    }))
}
