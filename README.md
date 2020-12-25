# Rust Untyped Lambda Calculus

This is an implementation of a simple [untyped lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus) in Rust.

I wrote this in December 2020 as a way to play with Rust, Nom, and brush up on Lambda Calculus basics.

Unfortunately, only relatively simple expressions work - notably, the Y combinator will cause a stack overflow.

## Usage

1. `cargo run examples/X` to compile the program and run a particular example.

## Sample outputs

```
$ cargo run examples/succ0.lc

Parsed:
((λn.(λf.(λx.(f ((n f) x))))) (λf.(λx.x)))

Reduced:
(λf.(λx.(f x)))
```

```
$ cargo run examples/basic_math.lc

Parsed:
SUCC := (λn.(λf.(λx.(f ((n f) x)))))
PRED := (λn.(λf.(λx.(((n (λg.(λh.(h (g f))))) (λu.x)) (λu.u)))))
PLUS := (λm.(λn.(λf.(λx.((m f) ((n f) x))))))
MULT := (λm.(λn.(λf.(m (n f)))))
POW := (λx.(λy.(y x)))
0 := (λf.(λx.x))
1 := (SUCC 0)
2 := (SUCC 1)
3 := (SUCC 2)
4 := (SUCC 3)
5 := (SUCC 4)
6 := (SUCC 5)
7 := (SUCC 6)
8 := (SUCC 7)
9 := (SUCC 8)
10 := (SUCC 9)
20 := ((PLUS 10) 10)
30 := ((MULT 10) 3)
10
20
(PRED 20)
30
((POW 2) 8)

Reduced:
(λf.(λx.(f (f (f (f (f (f (f (f (f (f x))))))))))))
(λf.(λx.(f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f x))))))))))))))))))))))
(λf.(λx.(f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f x)))))))))))))))))))))
(λf.(λx.(f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f (f x))))))))))))))))))))))))))))))))
(λx.(λbw.(x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x (x bw))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
```


```
$ cargo run examples/basic_logic.lc
Parsed:
TRUE := (λx.(λy.x))
FALSE := (λx.(λy.y))
AND := (λp.(λq.((p q) p)))
OR := (λp.(λq.((p p) q)))
NOT := (λp.((p FALSE) TRUE))
IFTHENELSE := (λp.(λa.(λb.((p a) b))))
SUCC := (λn.(λf.(λx.(f ((n f) x)))))
PRED := (λn.(λf.(λx.(((n (λg.(λh.(h (g f))))) (λu.x)) (λu.u)))))
PLUS := (λm.(λn.(λf.(λx.((m f) ((n f) x))))))
SUB := (λm.(λn.((n PRED) m)))
MULT := (λm.(λn.(λf.(m (n f)))))
POW := (λx.(λy.(y x)))
ISZERO := (λn.((n (λx.FALSE)) TRUE))
LEQ := (λm.(λn.(ISZERO ((SUB m) n))))
0 := (λf.(λx.x))
1 := (SUCC 0)
2 := (SUCC 1)
3 := (SUCC 2)
4 := (SUCC 3)
5 := (SUCC 4)
6 := (SUCC 5)
7 := (SUCC 6)
8 := (SUCC 7)
9 := (SUCC 8)
10 := (SUCC 9)
((AND TRUE) FALSE)
((OR TRUE) FALSE)
((LEQ 10) 1)
((LEQ 1) 10)

Reduced:
(λx.(λy.y))
(λx.(λas.x))
(λx.(λy.y))
(λx.(λy.x))
```
