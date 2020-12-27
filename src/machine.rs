use crate::term::{Term, TermOrDef, Program};

use std::collections::HashMap;
use std::rc::Rc;

const MAX_RECURSION_DEPTH: i16 = 10;

fn count_max_binders(term: &Term) -> i16 {
    match term {
        Term::Var(_) => 0,
        Term::Abstraction(_, t) => 1 + count_max_binders(t),
        Term::Application(t1, t2) => count_max_binders(t1) + count_max_binders(t2)
    }
}

// Debruijn-index a term
fn debruijn(term: Rc<Term>, prefix: &str, initial: i16, hm: &HashMap<String, Rc<Term>>) -> Rc<Term> {
    let num_binders = count_max_binders(&*term);
    match &*term {
        Term::Var(_) => term,
        Term::Abstraction(param, body) => {
            let db_index = prefix.to_string() + &(num_binders + initial).to_string();
            Rc::new(Term::Abstraction(db_index.clone(), debruijn(rewrite(body.clone(), &param, &db_index), prefix, initial, hm)))
        },
        Term::Application(t1, t2) => {
            let lhs_initial = initial + count_max_binders(&*t2);
            let lhs = debruijn(t1.clone(), prefix, lhs_initial, hm);
            let rhs = debruijn(t2.clone(), prefix, initial, hm);
            Rc::new(Term::Application(lhs, rhs))
        }
    }
}

fn perform_lookups(term: Rc<Term>, hm: &HashMap<String, Rc<Term>>) -> Rc<Term> {
    match &*term {
        Term::Var(name) => {
            if hm.contains_key(name) {
                hm.get(name).unwrap().clone()
            } else {
                term
            }
        },
        Term::Abstraction(param, body) => {
            Rc::new(Term::Abstraction(param.to_string(), perform_lookups(body.clone(), hm)))
        },
        Term::Application(t1, t2) => Rc::new(Term::Application(perform_lookups(t1.clone(), hm), perform_lookups(t2.clone(), hm)))
    }
}

// alpha reduction
fn rewrite(term: Rc<Term>, var: &str, new_var: &str) -> Rc<Term> {
    match &*term {
        Term::Var(name) => {
            if name == var {
                Rc::new(Term::Var(new_var.to_string()))
            } else {
                term
            }
        },
        Term::Abstraction(param, body) => {
            if param == var {
                term
            } else {
                Rc::new(Term::Abstraction(param.to_string(), rewrite(body.clone(), var, new_var)))
            }
        },
        Term::Application(t1, t2) => {
            let t1p = rewrite(t1.clone(), var, new_var);
            let t2p = rewrite(t2.clone(), var, new_var);
            Rc::new(Term::Application(t1p, t2p))
        }
    }
}

fn replace(term: Rc<Term>, var: &str, substitution: Rc<Term>, hm: &HashMap<String, Rc<Term>>, depth: i16) -> Rc<Term> {
    match &*term {
        Term::Var(name) => {
            if name == var {
                substitution.clone()
            } else {
                term.clone()
            }
        },
        Term::Abstraction(param, body) => {
            if param == var {
                term
            } else {
                Rc::new(Term::Abstraction(param.to_string(), replace(body.clone(), var, substitution, hm, depth+1)))
            }
        },
        Term::Application(t1, t2) => {
            let t1p = replace(t1.clone(), var, substitution.clone(), hm, depth);
            let t2p = replace(t2.clone(), var, substitution.clone(), hm, depth);
            // Prevent infinite recursion
            let tp = Rc::new(Term::Application(t1p, t2p));
            if depth > MAX_RECURSION_DEPTH {
                tp
            } else {
                reduce_term(tp, hm, depth+1)
            }
        }
    }
}

pub fn reduce_term(term: Rc<Term>, hm: &HashMap<String, Rc<Term>>, depth: i16) -> Rc<Term> {
    if depth > MAX_RECURSION_DEPTH {
        // Prevent infinite recursion
        term
    } else {
        match &*term {
            Term::Application(t1, t2) => match &**t1 {
                Term::Abstraction(param, body) => {
                    let replaced_term = replace(body.clone(), &param, t2.clone(), hm, depth);
                    reduce_term(replaced_term, hm, depth+1)
                },
                _ => {
                    let t1p = reduce_term(t1.clone(), hm, depth);
                    let t2p = reduce_term(t2.clone(), hm, depth);
                    Rc::new(Term::Application(t1p, t2p))
                }
            },
            Term::Abstraction(param, body) => {
                Rc::new(Term::Abstraction(param.to_string(), reduce_term(body.clone(), hm, depth+1)))
            },
            _ => term
        }
    }
}

pub fn reduce(program: &Program) -> Program {
    let mut definitions: HashMap<String, Rc<Term>> = HashMap::new();

    Program(program.0.iter().fold(Vec::new(), |mut v, term| {
        match term {
            TermOrDef::Definition(name, term) => {
                definitions.insert(name.to_string(), Rc::new(term.clone()));
                v
            }
            TermOrDef::Term(t) => {
                let mut prev = Rc::new(t.clone());
                let mut substituted = perform_lookups(prev.clone(), &definitions);
                // Fully resolve referenced nicknames
                while *substituted != *prev {
                    prev = substituted.clone();
                    substituted = perform_lookups(prev.clone(), &definitions);
                }
                prev = substituted.clone();
                let mut reduced = debruijn(substituted, "_", 0, &definitions);
                // Reduce repeatedly in a loop at the highest level, instead of
                // relying on deep recursion within `reduce_term` which can cause
                // stack overflows.
                // This method is very slow and relies on lots of copying, but this is
                // a toy, after all.
                while *reduced != *prev {
                    prev = reduced;
                    reduced = debruijn(reduce_term(prev.clone(), &definitions, 0), "_", 0, &definitions);
                }
                let ta = debruijn(reduced, "", 0, &definitions);
                println!("REDUCED: t {} reduced to {}", t, ta);
                v.push(TermOrDef::Term((*ta).clone()));
                v
            }
        }
    }))
}
