#![allow(unused_imports)]
#![allow(unused_variables)]
use expr_arena_ast::{Arena, Node, Op};
use util::tok::Tok;

#[allow(non_snake_case)]
pub fn parse_Expr<
    'ast,
    __TOKENS: IntoIterator<Item=(usize,Tok,usize)>,
>(
    arena: &'ast Arena<'ast>,
    __tokens: __TOKENS,
) -> Result<(Option<(usize,Tok,usize)>, &'ast Node<'ast>), Option<(usize,Tok,usize)>>
{
    let mut __tokens = __tokens.into_iter();
    let __lookahead = __tokens.next();
    match try!(__parse__Expr::__state0(arena, None, __lookahead, &mut __tokens)) {
        (_, __lookahead, __parse__Expr::__Nonterminal::____Expr(__nt)) => Ok((__lookahead, __nt)),
        _ => unreachable!(),
    }
}

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use expr_arena_ast::{Arena, Node, Op};
    use util::tok::Tok;

    pub enum __Nonterminal<'ast, > {
        _28_3cExpr_3e_20_22_2c_22_29(&'ast Node<'ast>),
        Comma_3cExpr_3e(Vec<&'ast Node<'ast>>),
        Term(&'ast Node<'ast>),
        Factor(&'ast Node<'ast>),
        _28_3cExpr_3e_20_22_2c_22_29_2a(::std::vec::Vec<&'ast Node<'ast>>),
        Expr(&'ast Node<'ast>),
        Expr_3f(::std::option::Option<&'ast Node<'ast>>),
        ____Expr(&'ast Node<'ast>),
    }

    // State 0
    //   Expr = (*) Expr "+" Factor [EOF]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [EOF]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [EOF]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   "(" -> Shift(S3)
    //   "*" -> Shift(S6)
    //   "Num" -> Shift(S1)
    //
    //   Expr -> S4
    //   Term -> S5
    //   Factor -> S2
    pub fn __state0<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookbehind, __lookahead, __tokens, __sym0));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookbehind, __lookahead, __tokens, __sym0));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym0));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(arena, __lookbehind, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(arena, __lookbehind, __lookahead, __tokens, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(arena, __lookbehind, __lookahead, __tokens, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Term = "Num" (*) [EOF]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   EOF -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //
    pub fn __state1<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 2
    //   Expr = Factor (*) [EOF]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S8)
    //   "*" -> Shift(S7)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   EOF -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //
    pub fn __state2<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state7(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [EOF]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S14)
    //   "*" -> Shift(S11)
    //   "Num" -> Shift(S9)
    //
    //   Expr -> S10
    //   Term -> S12
    //   Factor -> S13
    pub fn __state3<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state11(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   "-" -> Shift(S15)
    //   "+" -> Shift(S16)
    //   EOF -> Reduce(__Expr = Expr => Call(ActionFn(0));)
    //
    pub fn __state4<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state15(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state16(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 5
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   EOF -> Reduce(Factor = Term => Call(ActionFn(7));)
    //
    pub fn __state5<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 6
    //   Factor = "*" (*) "(" Comma<Expr> ")" [EOF]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["*"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["+"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["-"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["/"]
    //
    //   "(" -> Shift(S17)
    //
    pub fn __state6<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state17(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 7
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S3)
    //   "Num" -> Shift(S1)
    //
    //   Term -> S18
    pub fn __state7<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S3)
    //   "Num" -> Shift(S1)
    //
    //   Term -> S19
    pub fn __state8<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state19(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   Term = "Num" (*) [")"]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   ")" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //
    pub fn __state9<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 10
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [EOF]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "+" -> Shift(S22)
    //   ")" -> Shift(S21)
    //   "-" -> Shift(S20)
    //
    pub fn __state10<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state22(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state21(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state20(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 11
    //   Factor = "*" (*) "(" Comma<Expr> ")" [")"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["*"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["+"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["-"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["/"]
    //
    //   "(" -> Shift(S23)
    //
    pub fn __state11<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state23(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   ")" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //
    pub fn __state12<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 13
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "*" -> Shift(S25)
    //   ")" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S24)
    //
    pub fn __state13<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state25(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state24(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 14
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "*" -> Shift(S11)
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S14)
    //
    //   Expr -> S26
    //   Term -> S12
    //   Factor -> S13
    pub fn __state14<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state11(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   Expr = Expr "-" (*) Factor [EOF]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S3)
    //   "Num" -> Shift(S1)
    //   "*" -> Shift(S6)
    //
    //   Term -> S5
    //   Factor -> S27
    pub fn __state15<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   Expr = Expr "+" (*) Factor [EOF]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [EOF]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [EOF]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S3)
    //   "Num" -> Shift(S1)
    //   "*" -> Shift(S6)
    //
    //   Factor -> S28
    //   Term -> S5
    pub fn __state16<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state3(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state1(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state6(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) ["Num"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["Num"]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [EOF]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   "Num" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "(" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   ")" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "*" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //
    //   (<Expr> ",")* -> S30
    //   Comma<Expr> -> S29
    pub fn __state17<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(_), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::LParen(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::Times(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state18<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 19
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   EOF -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state19<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 20
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S14)
    //   "Num" -> Shift(S9)
    //   "*" -> Shift(S11)
    //
    //   Factor -> S31
    //   Term -> S12
    pub fn __state20<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state11(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state31(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 21
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   EOF -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //
    pub fn __state21<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 22
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "*" -> Shift(S11)
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S14)
    //
    //   Factor -> S32
    //   Term -> S12
    pub fn __state22<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state11(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 23
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) ["Num"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["Num"]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   "(" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   ")" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "*" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "Num" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //
    //   Comma<Expr> -> S33
    //   (<Expr> ",")* -> S30
    pub fn __state23<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::LParen(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::Times(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::Num(_), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 24
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S14)
    //   "Num" -> Shift(S9)
    //
    //   Term -> S34
    pub fn __state24<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S14)
    //   "Num" -> Shift(S9)
    //
    //   Term -> S35
    pub fn __state25<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state35(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 26
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "+" -> Shift(S22)
    //   ")" -> Shift(S36)
    //   "-" -> Shift(S20)
    //
    pub fn __state26<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state22(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state36(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state20(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 27
    //   Expr = Expr "-" Factor (*) [EOF]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "/" -> Shift(S8)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   EOF -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "*" -> Shift(S7)
    //
    pub fn __state27<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state7(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 28
    //   Expr = Expr "+" Factor (*) [EOF]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S8)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S7)
    //   EOF -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //
    pub fn __state28<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state8(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state7(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Factor = "*" "(" Comma<Expr> (*) ")" [EOF]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["*"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["+"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["-"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["/"]
    //
    //   ")" -> Shift(S37)
    //
    pub fn __state29<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state37(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 30
    //   (<Expr> ",") = (*) Expr "," ["("]
    //   (<Expr> ",") = (*) Expr "," [")"]
    //   (<Expr> ",") = (*) Expr "," ["*"]
    //   (<Expr> ",") = (*) Expr "," ["Num"]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["("]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") [")"]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (<Expr> ",")* (*) (<Expr> ",") ["Num"]
    //   Comma<Expr> = (<Expr> ",")* (*) Expr? [")"]
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor [","]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor [","]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor [","]
    //   Expr = (*) Factor ["-"]
    //   Expr? = (*) [")"]
    //   Expr? = (*) Expr [")"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term [","]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term [","]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term [","]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [","]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S45)
    //   ")" -> Reduce(Expr? =  => Call(ActionFn(12));)
    //   "Num" -> Shift(S42)
    //   "*" -> Shift(S40)
    //
    //   Factor -> S38
    //   Expr -> S41
    //   Expr? -> S39
    //   Term -> S43
    //   (<Expr> ",") -> S44
    pub fn __state30<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state45(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state42(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state40(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __nt = super::__action12(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Expr_3f(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state38(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state41(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr_3f(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state39(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state43(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state44(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 31
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "/" -> Shift(S24)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "*" -> Shift(S25)
    //
    pub fn __state31<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state24(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state25(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 32
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S24)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S25)
    //
    pub fn __state32<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state24(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state25(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 33
    //   Factor = "*" "(" Comma<Expr> (*) ")" [")"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["*"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["+"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["-"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["/"]
    //
    //   ")" -> Shift(S46)
    //
    pub fn __state33<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state46(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 34
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   ")" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state34<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 35
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   ")" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state35<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 36
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //
    pub fn __state36<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 37
    //   Factor = "*" "(" Comma<Expr> ")" (*) [EOF]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["*"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["+"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["-"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["/"]
    //
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   EOF -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //
    pub fn __state37<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 38
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) [","]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term [","]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term [","]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "," -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "*" -> Shift(S47)
    //   "/" -> Shift(S48)
    //   ")" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //
    pub fn __state38<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state47(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state48(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 39
    //   Comma<Expr> = (<Expr> ",")* Expr? (*) [")"]
    //
    //   ")" -> Reduce(Comma<Expr> = (<Expr> ",")*, Expr? => Call(ActionFn(10));)
    //
    pub fn __state39<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
        __sym1: &mut Option<::std::option::Option<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action10(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Comma_3cExpr_3e(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 40
    //   Factor = "*" (*) "(" Comma<Expr> ")" [")"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["*"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["+"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" [","]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["-"]
    //   Factor = "*" (*) "(" Comma<Expr> ")" ["/"]
    //
    //   "(" -> Shift(S49)
    //
    pub fn __state40<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state49(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 41
    //   (<Expr> ",") = Expr (*) "," ["("]
    //   (<Expr> ",") = Expr (*) "," [")"]
    //   (<Expr> ",") = Expr (*) "," ["*"]
    //   (<Expr> ",") = Expr (*) "," ["Num"]
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor [","]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor [","]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Expr? = Expr (*) [")"]
    //
    //   "+" -> Shift(S50)
    //   ")" -> Reduce(Expr? = Expr => Call(ActionFn(11));)
    //   "," -> Shift(S51)
    //   "-" -> Shift(S52)
    //
    pub fn __state41<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state50(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Comma(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state51(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state52(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr_3f(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 42
    //   Term = "Num" (*) [")"]
    //   Term = "Num" (*) ["*"]
    //   Term = "Num" (*) ["+"]
    //   Term = "Num" (*) [","]
    //   Term = "Num" (*) ["-"]
    //   Term = "Num" (*) ["/"]
    //
    //   "/" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "," -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   ")" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "Num" => Call(ActionFn(8));)
    //
    pub fn __state42<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 43
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) [","]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "," -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(7));)
    //
    pub fn __state43<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(arena, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 44
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["("]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) [")"]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["*"]
    //   (<Expr> ",")* = (<Expr> ",")* (<Expr> ",") (*) ["Num"]
    //
    //   "*" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(14));)
    //   ")" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(14));)
    //   "(" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(14));)
    //   "Num" -> Reduce((<Expr> ",")* = (<Expr> ",")*, (<Expr> ",") => Call(ActionFn(14));)
    //
    pub fn __state44<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'ast Node<'ast>>>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            Some((_, Tok::LParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action14(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 45
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" [","]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S9)
    //   "(" -> Shift(S14)
    //   "*" -> Shift(S11)
    //
    //   Factor -> S13
    //   Expr -> S53
    //   Term -> S12
    pub fn __state45<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state9(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state14(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state11(arena, __lookbehind, __lookahead, __tokens, __sym1));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state53(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(arena, __lookbehind, __lookahead, __tokens, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 46
    //   Factor = "*" "(" Comma<Expr> ")" (*) [")"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["*"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["+"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["-"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["/"]
    //
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   ")" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //
    pub fn __state46<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 47
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term [","]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S42)
    //   "(" -> Shift(S45)
    //
    //   Term -> S54
    pub fn __state47<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state42(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state45(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state54(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 48
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term [","]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "Num" -> Shift(S42)
    //   "(" -> Shift(S45)
    //
    //   Term -> S55
    pub fn __state48<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state42(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state45(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 49
    //   (<Expr> ",")* = (*) ["("]
    //   (<Expr> ",")* = (*) [")"]
    //   (<Expr> ",")* = (*) ["*"]
    //   (<Expr> ",")* = (*) ["Num"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["("]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") [")"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["*"]
    //   (<Expr> ",")* = (*) (<Expr> ",")* (<Expr> ",") ["Num"]
    //   Comma<Expr> = (*) (<Expr> ",")* Expr? [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [")"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["*"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["+"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" [","]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["-"]
    //   Factor = "*" "(" (*) Comma<Expr> ")" ["/"]
    //
    //   ")" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "Num" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "*" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //   "(" -> Reduce((<Expr> ",")* =  => Call(ActionFn(13));)
    //
    //   (<Expr> ",")* -> S30
    //   Comma<Expr> -> S56
    pub fn __state49<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::Num(_), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::Times(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            Some((_, Tok::LParen(..), _)) => {
                let __nt = super::__action13(arena, );
                __result = (__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29_2a(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Comma_3cExpr_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 50
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor [","]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term [","]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term [","]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term [","]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [","]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S45)
    //   "*" -> Shift(S40)
    //   "Num" -> Shift(S42)
    //
    //   Factor -> S57
    //   Term -> S43
    pub fn __state50<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state45(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state40(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state42(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state57(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state43(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 51
    //   (<Expr> ",") = Expr "," (*) ["("]
    //   (<Expr> ",") = Expr "," (*) [")"]
    //   (<Expr> ",") = Expr "," (*) ["*"]
    //   (<Expr> ",") = Expr "," (*) ["Num"]
    //
    //   "(" -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(15));)
    //   "Num" -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(15));)
    //   ")" -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(15));)
    //   "*" -> Reduce((<Expr> ",") = Expr, "," => Call(ActionFn(15));)
    //
    pub fn __state51<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::LParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action15(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action15(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action15(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action15(arena, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cExpr_3e_20_22_2c_22_29(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 52
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor [","]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term [","]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term [","]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term [","]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [")"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["*"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["+"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" [","]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["-"]
    //   Factor = (*) "*" "(" Comma<Expr> ")" ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" [","]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) "Num" [")"]
    //   Term = (*) "Num" ["*"]
    //   Term = (*) "Num" ["+"]
    //   Term = (*) "Num" [","]
    //   Term = (*) "Num" ["-"]
    //   Term = (*) "Num" ["/"]
    //
    //   "(" -> Shift(S45)
    //   "*" -> Shift(S40)
    //   "Num" -> Shift(S42)
    //
    //   Term -> S43
    //   Factor -> S58
    pub fn __state52<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state45(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state40(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                let __lookahead = __tokens.next();
                __result = try!(__state42(arena, __lookbehind, __lookahead, __tokens, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state43(arena, __lookbehind, __lookahead, __tokens, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 53
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" [","]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "-" -> Shift(S20)
    //   "+" -> Shift(S22)
    //   ")" -> Shift(S59)
    //
    pub fn __state53<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state20(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state22(arena, __lookbehind, __lookahead, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state59(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 54
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) [","]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   ")" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "," -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state54<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 55
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) [","]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   ")" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "," -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state55<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 56
    //   Factor = "*" "(" Comma<Expr> (*) ")" [")"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["*"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["+"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" [","]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["-"]
    //   Factor = "*" "(" Comma<Expr> (*) ")" ["/"]
    //
    //   ")" -> Shift(S60)
    //
    pub fn __state56<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state60(arena, __lookbehind, __lookahead, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 57
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) [","]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term [","]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term [","]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "," -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S47)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S48)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //
    pub fn __state57<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state47(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state48(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 58
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) [","]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term [","]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term [","]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "*" -> Shift(S47)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "," -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "/" -> Shift(S48)
    //
    pub fn __state58<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'ast Node<'ast>>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<&'ast Node<'ast>>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state47(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                let __lookahead = __tokens.next();
                __result = try!(__state48(arena, __lookbehind, __lookahead, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
        return Ok(__result);
    }

    // State 59
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) [","]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //   "," -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(9));)
    //
    pub fn __state59<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<&'ast Node<'ast>>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(arena, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }

    // State 60
    //   Factor = "*" "(" Comma<Expr> ")" (*) [")"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["*"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["+"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) [","]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["-"]
    //   Factor = "*" "(" Comma<Expr> ")" (*) ["/"]
    //
    //   "/" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "," -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   ")" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = "*", "(", Comma<Expr>, ")" => Call(ActionFn(6));)
    //
    pub fn __state60<
        'ast,
        __TOKENS: Iterator<Item=(usize,Tok,usize)>,
    >(
        arena: &'ast Arena<'ast>,
        __lookbehind: Option<usize>,
        __lookahead: Option<(usize,Tok,usize)>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<Vec<&'ast Node<'ast>>>,
        __sym3: &mut Option<Tok>,
    ) -> Result<(Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >), Option<(usize,Tok,usize)>>
    {
        let mut __result: (Option<usize>, Option<(usize,Tok,usize)>, __Nonterminal<'ast, >);
        match __lookahead {
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Comma(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Times(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::RParen(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action6(arena, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__lookahead);
            }
        }
    }
}

pub fn __action0<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action1<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    l: &'ast Node<'ast>,
    _: Tok,
    r: &'ast Node<'ast>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Sub, l, r))
}

pub fn __action2<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    l: &'ast Node<'ast>,
    _: Tok,
    r: &'ast Node<'ast>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Add, l, r))
}

pub fn __action3<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action4<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    l: &'ast Node<'ast>,
    _: Tok,
    r: &'ast Node<'ast>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Mul, l, r))
}

pub fn __action5<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    l: &'ast Node<'ast>,
    _: Tok,
    r: &'ast Node<'ast>,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Binary(Op::Div, l, r))
}

pub fn __action6<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    _: Tok,
    _: Tok,
    __0: Vec<&'ast Node<'ast>>,
    _: Tok,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Reduce(Op::Mul, __0))
}

pub fn __action7<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action8<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    n: i32,
) -> &'ast Node<'ast>
{
    arena.alloc(Node::Value(n))
}

pub fn __action9<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    _: Tok,
    __0: &'ast Node<'ast>,
    _: Tok,
) -> &'ast Node<'ast>
{
    (__0)
}

pub fn __action10<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    h: ::std::vec::Vec<&'ast Node<'ast>>,
    t: ::std::option::Option<&'ast Node<'ast>>,
) -> Vec<&'ast Node<'ast>>
{
    h.into_iter().chain(t).collect()
}

pub fn __action11<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
) -> ::std::option::Option<&'ast Node<'ast>>
{
    Some(__0)
}

pub fn __action12<
    'ast,
>(
    arena: &'ast Arena<'ast>,
) -> ::std::option::Option<&'ast Node<'ast>>
{
    None
}

pub fn __action13<
    'ast,
>(
    arena: &'ast Arena<'ast>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    vec![]
}

pub fn __action14<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    v: ::std::vec::Vec<&'ast Node<'ast>>,
    e: &'ast Node<'ast>,
) -> ::std::vec::Vec<&'ast Node<'ast>>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action15<
    'ast,
>(
    arena: &'ast Arena<'ast>,
    __0: &'ast Node<'ast>,
    _: Tok,
) -> &'ast Node<'ast>
{
    (__0)
}