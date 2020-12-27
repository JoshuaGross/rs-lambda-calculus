use crate::term::{Term, TermOrDef, Program};
use crate::name_gen::{NameGen};

use std::collections::HashMap;

fn is_free(term: &Term, var: &str) -> bool {
    match term {
        Term::Var(var2) => var == var2,
        //Term::Var(var2) => var != var2,
        Term::Abstraction(name, term) => (name != var) && is_free(term, var),
        Term::Application(t1, t2) => is_free(t1, var) && is_free(t2, var)
    }
}

fn is_used(term: &Term, var: &str) -> bool {
    match term {
        Term::Var(var2) => var == var2,
        Term::Abstraction(name, term) => (name == var) || is_used(term, var),
        Term::Application(t1, t2) => is_used(t1, var) || is_used(t2, var)
    }
}

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
        Term::Var(name) => {
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
        Term::Var(name) => {
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
                // println!("PERFORM_LOOKUP REWRITE: {} {}", name, hm.get(name).unwrap().clone());
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
    // println!("Alpha reduction: rewrite: {}->{} {}", var, new_var, term);

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

fn replace(term: &Term, var: &str, substitution: &Term, ng: &mut NameGen, hm: &HashMap<String, Term>, depth: i16) -> Term {
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
                // println!("replace 1 {}", depth);
                Term::Abstraction(param.to_string(), Box::new(replace(body, var, substitution, ng, hm, depth+1)))
            }
                /* else if is_free(substitution, param) {
                Term::Abstraction(param.to_string(), Box::new(replace(body, var, substitution, ng, hm)))
            } else {
                let mut rewritten_param = ng.next().unwrap().to_string();
                while is_used(substitution, &rewritten_param) {
                    rewritten_param = ng.next().unwrap().to_string();
                }
                Term::Abstraction(param.to_string(), Box::new(replace(body, var, &rewrite(substitution, param, &rewritten_param), ng, hm)))
            }*/
        },
        Term::Application(t1, t2) => {
            let t1p = Box::new(replace(&(**t1), var, substitution, ng, hm, depth+1));
            let t2p = Box::new(replace(&(**t2), var, substitution, ng, hm, depth+1));
            // println!("replace 2 {}", depth);
            // Prevent infinite recursion
            let t = Term::Application(t1p, t2p);
            let reduced = if depth > 100 {
                t
            } else {
                reduce_term(&t, ng, hm, depth+1)
            };
            reduced
            // let t = reduce_term(&Term::Application(t1p, t2p), ng, hm, depth+1);
            // println!("replace {} in term App({}, {}), after reduce: {}", var, t1, t2, t);
            // t
        }
    }
}

pub fn reduce_term(term: &Term, ng: &mut NameGen, hm: &HashMap<String, Term>, depth: i16) -> Term {
    // println!("reduce_term1 before lookup: {}", term);
    // let term = debruijn(&perform_lookups(term, hm), 0, hm);
    // let term = perform_lookups(term, hm);
    // println!("reduce_term2 after  lookup: {}", term);
    let term_copied = term.clone();
    // println!("reduce_term before: {}", depth, /* term */);

    let reduced_term = match term {
        Term::Application(t1, t2) => match &**t1 {
            Term::Abstraction(param, body) => {
                // println!("reduce_term3 app+abs: {} {} {}", param, db_body, db_t2);

                // Prevent infinite recursion
                // if db_body != db_t2 {
                let replaced_term = replace(&Box::new(body), &param, &Box::new(t2), ng, hm, depth+1);
                // Prevent infinite recursion
                if depth > 1000 {
                    replaced_term
                } else {
                    reduce_term(&replaced_term, ng, hm, depth+1)
                }

                // } else {
                //     Term::Application(Box::new(Term::Abstraction(param, body)), t2)
                // }

                //replace(&Box::new(body), &param, &Box::new(reduce_term(&t2, ng, hm)), ng, hm)
                // reduce_term(&replace(&Box::new(body), &param, &Box::new(reduce_term(&t2, ng, hm)), ng, hm), ng, hm)
            },
            _ => {
                // if (depth > 1000) {
                    // Prevent infinite recursion
                //     Term::Application(t1.clone(), t2.clone())
                // } else {
                    Term::Application(Box::new(reduce_term(&t1, ng, hm, depth+1)), Box::new(reduce_term(&t2, ng, hm, depth+1)))
                // }
            }
            // _ => Term::Application(Box::new(reduce_term(&t1, ng, hm, depth+1)), Box::new(reduce_term(&t2, ng, hm, depth+1)))
            /*
            // {
            //     println!("reduce_term: {} {}", t1, t2);
            //     let t1p = reduce_term(&t1, ng, hm);
            //     let t2p = reduce_term(&t2, ng, hm);
            //     Term::Application(Box::new(t1p), Box::new(t2p))
            // }
            */
        },
        Term::Abstraction(param, body) => {
            // if depth > 1000 {
                // Prevent infinite recursion
            //     term.clone()
            // } else {
                Term::Abstraction(param.to_string(), Box::new(reduce_term(&body, ng, hm, depth+1)))
            // }
        },
        _ => term.clone()
    };
    // println!("reduce_term after: {} // {}", term_copied, reduced_term);
    reduced_term
}

pub fn reduce(program: &Program, ng: &mut NameGen) -> Program {
    let mut definitions: HashMap<String, Term> = HashMap::new();

    Program(program.0.iter().fold(Vec::new(), |mut v, term| {
        match term {
            TermOrDef::Definition(name, term) => {
                // definitions.insert(name.to_string(), reduce_term(&term, ng, &definitions));
                definitions.insert(name.to_string(), term.clone());
                v
            }
            TermOrDef::Term(t) => {
                println!("REDUCING: t {}", t);
                let mut prev = t.clone();
                let mut substituted = perform_lookups(&t, &definitions);
                // Fully resolve referenced nicknames
                while substituted != prev {
                    println!("RE-LOOKUP: {} {}", prev, substituted);
                    prev = substituted;
                    substituted = perform_lookups(&prev, &definitions);
                }
                prev = substituted.clone();
                let mut reduced = debruijn(&substituted, 0, &definitions);
                println!("RE-INDEXED: {}", reduced);
                // Reduce repeatedly in a loop at the highest level, instead of
                // relying on deep recursion within `reduce_term` which can cause
                // stack overflows.
                // This method is very slow and relies on lots of copying, but this is
                // a toy, after all.
                while reduced != prev {
                    println!("RE-REDUCING: {}", reduced.clone());
                    prev = reduced;
                    reduced = debruijn(&reduce_term(&prev, ng, &definitions, 0), 0, &definitions);
                    // println!("Re-reduced: tttt {}", t);
                    // println!("Re-reduced: prev {}", prev);
                    // println!("Re-reduced: redu {}", reduced);
                }
                println!("REDUCED: reduced {}", reduced);
                v.push(TermOrDef::Term(debruijn_aesthetic(&reduced, 0, &definitions)));
                v
            }
        }
    }))
}
