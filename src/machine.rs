use crate::term::{Term, TermOrDef, Program};

use std::collections::HashMap;

const MAX_RECURSION_DEPTH: i16 = 10;

fn count_max_binders(term: &Term) -> i16 {
    match term {
        Term::Var(_) => 0,
        Term::Abstraction(_, t1) => 1 + count_max_binders(t1),
        Term::Application(t1, t2) => count_max_binders(t1) + count_max_binders(t2)
    }
}

// Debruijn-index a term
fn debruijn(term: &Term, initial: i16, hm: &HashMap<String, Term>) -> Term {
    let num_binders = count_max_binders(term);
    match term {
        Term::Var(_) => {
            term.clone()
        },
        Term::Abstraction(param, body) => {
            let db_index = "_".to_string() + &(num_binders + initial).to_string();
            Term::Abstraction(db_index.clone(), Box::new(debruijn(&rewrite(&Box::new(body), param, &db_index), initial, hm)))
        },
        Term::Application(t1, t2) => {
            let lhs_initial = initial + count_max_binders(t2);
            Term::Application(Box::new(debruijn(t1, lhs_initial, hm)), Box::new(debruijn(t2, initial, hm)))
        }
    }
}

fn debruijn_aesthetic(term: &Term, initial: i16, hm: &HashMap<String, Term>) -> Term {
    let num_binders = count_max_binders(term);
    match term {
        Term::Var(_) => {
            term.clone()
        },
        Term::Abstraction(param, body) => {
            let db_index = &num_binders.to_string();
            Term::Abstraction(db_index.clone(), Box::new(debruijn_aesthetic(&rewrite(&Box::new(body), param, &db_index), initial, hm)))
        },
        Term::Application(t1, t2) => {
            let lhs_initial = initial + count_max_binders(t2);
            Term::Application(Box::new(debruijn_aesthetic(t1, lhs_initial, hm)), Box::new(debruijn_aesthetic(t2, initial, hm)))
        }
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
                term.clone()
                // Term::Abstraction(new_var.to_string(), Box::new(rewrite(body, var, new_var)))
            } else {
                Term::Abstraction(param.to_string(), Box::new(rewrite(body, var, new_var)))
            }
        },
        Term::Application(t1, t2) => Term::Application(Box::new(rewrite(&(**t1), var, new_var)), Box::new(rewrite(&(**t2), var, new_var)))
    }
}

fn replace(term: &Term, var: &str, substitution: &Term, hm: &HashMap<String, Term>, depth: i16) -> Term {
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
            } else {
                Term::Abstraction(param.to_string(), Box::new(replace(body, var, substitution, hm, depth+1)))
            }
        },
        Term::Application(t1, t2) => {
            let t1p = Box::new(replace(&(**t1), var, substitution, hm, depth));
            let t2p = Box::new(replace(&(**t2), var, substitution, hm, depth));
            // Prevent infinite recursion
            let t = Term::Application(t1p, t2p);
            let reduced = if depth > MAX_RECURSION_DEPTH {
                t
            } else {
                reduce_term(&t, hm, depth+1)
            };
            reduced
        }
    }
}

pub fn reduce_term(term: &Term, hm: &HashMap<String, Term>, depth: i16) -> Term {
    if depth > MAX_RECURSION_DEPTH {
        // Prevent infinite recursion
        term.clone()
    } else {
        match term {
            Term::Application(t1, t2) => match &**t1 {
                Term::Abstraction(param, body) => {
                    let replaced_term = replace(&Box::new(body), &param, &Box::new(t2), hm, depth);
                    reduce_term(&replaced_term, hm, depth+1)
                },
                _ => {
                    Term::Application(Box::new(reduce_term(&t1, hm, depth)), Box::new(reduce_term(&t2, hm, depth)))
                }
            },
            Term::Abstraction(param, body) => {
                Term::Abstraction(param.to_string(), Box::new(reduce_term(&body, hm, depth+1)))
            },
            _ => term.clone()
        }
    }
}

pub fn reduce(program: &Program) -> Program {
    let mut definitions: HashMap<String, Term> = HashMap::new();

    Program(program.0.iter().fold(Vec::new(), |mut v, term| {
        match term {
            TermOrDef::Definition(name, term) => {
                definitions.insert(name.to_string(), term.clone());
                v
            }
            TermOrDef::Term(t) => {
                let mut prev = t.clone();
                let mut substituted = perform_lookups(&t, &definitions);
                // Fully resolve referenced nicknames
                while substituted != prev {
                    prev = substituted;
                    substituted = perform_lookups(&prev, &definitions);
                }
                prev = substituted.clone();
                let mut reduced = debruijn(&substituted, 0, &definitions);
                // Reduce repeatedly in a loop at the highest level, instead of
                // relying on deep recursion within `reduce_term` which can cause
                // stack overflows.
                // This method is very slow and relies on lots of copying, but this is
                // a toy, after all.
                while reduced != prev {
                    prev = reduced;
                    reduced = debruijn(&reduce_term(&prev, &definitions, 0), 0, &definitions);
                }
                let ta = debruijn_aesthetic(&reduced, 0, &definitions);
                println!("REDUCED: t {} reduced to {}", t, ta);
                v.push(TermOrDef::Term(ta));
                v
            }
        }
    }))
}
