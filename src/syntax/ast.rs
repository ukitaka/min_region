pub struct RegionVar {
    name: String
}

pub enum RegionSize {
    Finite(i32),
    Infinite
}

pub enum AllocationPoint {
    AtTop(RegionVar),
    AtBot(RegionVar),
    SAt(RegionVar),
}

pub struct BindingPoint {
    region: RegionVar,
    size: RegionSize
}

pub enum Constant {
    Nil,
    True,
    False,
    Number(i32)
}

pub struct Var {
    name: String
}

pub enum Pattern {
    Var(Var),
    Pair(Var, Var),
    Cons(Var, Var),
}

pub enum Expr<'a> {
    Let {
        pattern: Pattern,
        expr: &'a Expr<'a>,
        body: &'a Expr<'a>,
    },
    Pair {
        fst: &'a Expr<'a>,
        snd: &'a Expr<'a>,
        at: AllocationPoint,
    },
    Cons {
        head: &'a Expr<'a>,
        tail: &'a Expr<'a>,
        at: AllocationPoint,
    },
    Closure {
        var: Var,
        expr: &'a Expr<'a>,
        at: AllocationPoint,
    },
    Case {
        target: &'a Expr<'a>,
        pattern1: Pattern,
        body1: &'a Expr<'a>,
        pattern2: Pattern,
        body2: &'a Expr<'a>,
    },
    LetRec {
        name: String,
        fr_params: Vec<BindingPoint>,
        param: Var,
        at: AllocationPoint,
        body: &'a Expr<'a>,
        expr: &'a Expr<'a>,
    },
    App {
        func_name: String,
        ar_params: Vec<AllocationPoint>,
        at: AllocationPoint,
    },
    LetRegion {
        bindings: Vec<BindingPoint>,
        expr: &'a Expr<'a>,
    }
}
