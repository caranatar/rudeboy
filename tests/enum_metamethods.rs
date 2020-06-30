use rlua::Lua;
use rudeboy::{
    metamethods,
    user_data
};

#[test]
fn add() -> rlua::Result<()> {
    #[metamethods(Add)]
    #[user_data(MetaMethods)]
    #[derive(Clone, PartialEq, Eq, Copy, Debug)]
    enum Bit {
        Zero,
        One,
        Overflow
    }

    impl std::ops::Add for Bit {
        type Output = Self;
    
        fn add(self, other: Self) -> Self {
            match (self, other) {
                (Bit::Overflow, _) => Bit::Overflow,
                (_, Bit::Overflow) => Bit::Overflow,
                (Bit::Zero, rhs) => rhs,
                (lhs, Bit::Zero) => lhs,
                (Bit::One, Bit::One) => Bit::Overflow,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("zero", Bit::Zero)?;
        globals.set("one", Bit::One)?;

        let sum = ctx.load("zero + zero").eval::<Bit>()?;
        assert_eq!(sum, Bit::Zero);
        let sum = ctx.load("zero + one").eval::<Bit>()?;
        assert_eq!(sum, Bit::One);
        let sum = ctx.load("one + one").eval::<Bit>()?;
        assert_eq!(sum, Bit::Overflow);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn sub() -> rlua::Result<()> {
    #[metamethods(Sub)]
    #[user_data(MetaMethods)]
    #[derive(Clone, PartialEq, Eq, Copy, Debug)]
    enum Bit {
        Zero,
        One,
        Underflow
    }

    impl std::ops::Sub for Bit {
        type Output = Self;
    
        fn sub(self, other: Self) -> Self {
            match (self, other) {
                (Bit::Underflow, _) => Bit::Underflow,
                (_, Bit::Underflow) => Bit::Underflow,
                (lhs, Bit::Zero) => lhs,
                (Bit::One, Bit::One) => Bit::Zero,
                (Bit::Zero, Bit::One) => Bit::Underflow,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("zero", Bit::Zero)?;
        globals.set("one", Bit::One)?;

        let diff = ctx.load("zero - zero").eval::<Bit>()?;
        assert_eq!(diff, Bit::Zero);
        let diff = ctx.load("zero - one").eval::<Bit>()?;
        assert_eq!(diff, Bit::Underflow);
        let diff = ctx.load("one - one").eval::<Bit>()?;
        assert_eq!(diff, Bit::Zero);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn eq() -> rlua::Result<()> {
    #[metamethods(Eq)]
    #[user_data(MetaMethods)]
    #[derive(PartialEq, Clone, Debug)]
    enum Foo {
        Bar,
        Baz(u8),
        Qux{ x: f64, y: f64 }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_bar_one", Foo::Bar)?;
        globals.set("foo_bar_two", Foo::Bar)?;
        
        globals.set("foo_baz_5_one", Foo::Baz(5))?;
        globals.set("foo_baz_5_two", Foo::Baz(5))?;
        globals.set("foo_baz_23", Foo::Baz(23))?;
        
        globals.set("foo_qux_2_3_one", Foo::Qux { x: 2.0, y: 3.0 })?;
        globals.set("foo_qux_2_3_two", Foo::Qux { x: 2.0, y: 3.0 })?;
        globals.set("foo_qux_5_23", Foo::Qux { x: 5.0, y: 23.0 })?;

        let cmp = ctx.load("foo_bar_one == foo_bar_two").eval::<bool>()?;
        assert!(cmp);
        let cmp = ctx.load("foo_bar_one == foo_baz_5_one").eval::<bool>()?;
        assert!(!cmp);
        let cmp = ctx.load("foo_bar_one == foo_qux_2_3_one").eval::<bool>()?;
        assert!(!cmp);

        let cmp = ctx.load("foo_baz_5_one == foo_baz_5_two").eval::<bool>()?;
        assert!(cmp);
        let cmp = ctx.load("foo_baz_5_one == foo_baz_23").eval::<bool>()?;
        assert!(!cmp);
        let cmp = ctx.load("foo_baz_5_one == foo_qux_2_3_one").eval::<bool>()?;
        assert!(!cmp);

        let cmp = ctx.load("foo_qux_2_3_one == foo_qux_2_3_two").eval::<bool>()?;
        assert!(cmp);
        let cmp = ctx.load("foo_qux_2_3_one == foo_qux_5_23").eval::<bool>()?;
        assert!(!cmp);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn mul() -> rlua::Result<()> {
    #[metamethods(Mul)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq, Eq)]
    enum Bit {
        Zero,
        One,
    }

    impl std::ops::Mul for Bit {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            match self {
                Bit::Zero => Bit::Zero,
                Bit::One => other,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("zero", Bit::Zero)?;
        globals.set("one", Bit::One)?;

        let res = ctx.load("zero * zero").eval::<Bit>()?;
        assert_eq!(res, Bit::Zero);

        let res = ctx.load("one * one").eval::<Bit>()?;
        assert_eq!(res, Bit::One);

        let res = ctx.load("one * zero").eval::<Bit>()?;
        assert_eq!(res, Bit::Zero);

        let res = ctx.load("zero * one").eval::<Bit>()?;
        assert_eq!(res, Bit::Zero);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn div() -> rlua::Result<()> {
    #[metamethods(Div)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Int(i64),
        Float(f64),
    }

    impl std::ops::Div for Foo {
        type Output = Self;

        fn div(self, other: Self) -> Self {
            let out = match self {
                Foo::Int(i) => i as f64,
                Foo::Float(f) => f
            } / match other {
                Foo::Int(i) => i as f64,
                Foo::Float(f) => f,
            };
            Foo::Float(out)
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo::Int(23))?;
        globals.set("foo_two", Foo::Int(10))?;

        let res = ctx.load("foo_one / foo_two").eval::<Foo>()?;
        assert_eq!(res, Foo::Float(2.3));

        Ok(())
    })?;
    Ok(())
}

#[test]
fn mod_() -> rlua::Result<()> {
    #[metamethods(Mod)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Int(i64),
        Float(f64),
    }

    impl std::ops::Rem for Foo {
        type Output = Self;

        fn rem(self, other: Self) -> Self {
            match (self, other) {
                (Foo::Int(a), Foo::Int(b)) => Foo::Int(a % b),
                (Foo::Float(a), Foo::Float(b)) => Foo::Float(a / b),
                (Foo::Int(a), Foo::Float(b)) => Foo::Float(a as f64 / b),
                (Foo::Float(a), Foo::Int(b)) => Foo::Float(a / b as f64),
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo::Int(23))?;
        globals.set("foo_two", Foo::Int(10))?;

        let res = ctx.load("foo_one % foo_two").eval::<Foo>()?;
        assert_eq!(res, Foo::Int(3));

        globals.set("foo_one", Foo::Float(23.0))?;
        globals.set("foo_two", Foo::Float(10.0))?;

        let res = ctx.load("foo_one % foo_two").eval::<Foo>()?;
        assert_eq!(res, Foo::Float(2.3));

        Ok(())
    })?;
    Ok(())
}

#[test]
fn band() -> rlua::Result<()> {
    #[metamethods(BAnd)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Eight(u8),
        Sixteen(u16),
    }

    impl std::ops::BitAnd for Foo {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            match (self, other) {
                (Foo::Eight(a), Foo::Eight(b)) => Foo::Eight(a & b),
                (Foo::Sixteen(a), Foo::Sixteen(b)) => Foo::Sixteen(a & b),
                (Foo::Eight(a), Foo::Sixteen(b)) => Foo::Sixteen(a as u16 & b),
                (Foo::Sixteen(a), Foo::Eight(b)) => Foo::Sixteen(a & b as u16),
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("eight_one", Foo::Eight(0b000101))?;
        globals.set("eight_two", Foo::Eight(0b001010))?;
        globals.set("sixteen", Foo::Sixteen(0b100100))?;

        let res = ctx.load("eight_one & eight_two").eval::<Foo>()?;
        assert_eq!(res, Foo::Eight(0));
        let res = ctx.load("eight_one & sixteen").eval::<Foo>()?;
        assert_eq!(res, Foo::Sixteen(0b100));

        Ok(())
    })?;
    Ok(())
}

#[test]
fn bor() -> rlua::Result<()> {
    #[metamethods(BOr)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Eight(u8),
        Sixteen(u16),
    }

    impl std::ops::BitOr for Foo {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            match (self, other) {
                (Foo::Eight(a), Foo::Eight(b)) => Foo::Eight(a | b),
                (Foo::Sixteen(a), Foo::Sixteen(b)) => Foo::Sixteen(a | b),
                (Foo::Eight(a), Foo::Sixteen(b)) => Foo::Sixteen(a as u16 | b),
                (Foo::Sixteen(a), Foo::Eight(b)) => Foo::Sixteen(a | b as u16),
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("eight_one", Foo::Eight(0b000101))?;
        globals.set("eight_two", Foo::Eight(0b001010))?;
        globals.set("sixteen", Foo::Sixteen(0b100100))?;

        let res = ctx.load("eight_one | eight_two").eval::<Foo>()?;
        assert_eq!(res, Foo::Eight(0b001111));
        let res = ctx.load("eight_one | sixteen").eval::<Foo>()?;
        assert_eq!(res, Foo::Sixteen(0b100101));

        Ok(())
    })?;
    Ok(())
}

#[test]
fn bxor() -> rlua::Result<()> {
    #[metamethods(BXor)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Eight(u8),
        Sixteen(u16),
    }

    impl std::ops::BitXor for Foo {
        type Output = Self;

        fn bitxor(self, other: Self) -> Self {
            match (self, other) {
                (Foo::Eight(a), Foo::Eight(b)) => Foo::Eight(a ^ b),
                (Foo::Sixteen(a), Foo::Sixteen(b)) => Foo::Sixteen(a ^ b),
                (Foo::Eight(a), Foo::Sixteen(b)) => Foo::Sixteen(a as u16 ^ b),
                (Foo::Sixteen(a), Foo::Eight(b)) => Foo::Sixteen(a ^ b as u16),
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("sixteen", Foo::Sixteen(0b100100))?;
        globals.set("eight_one", Foo::Eight(0b000101))?;
        globals.set("eight_two", Foo::Eight(0b001010))?;

        let res = ctx.load("eight_one ~ eight_two").eval::<Foo>()?;
        assert_eq!(res, Foo::Eight(0b1111));
        let res = ctx.load("eight_one ~ sixteen").eval::<Foo>()?;
        assert_eq!(res, Foo::Sixteen(0b100001));

        Ok(())
    })?;
    Ok(())
}

#[test]
fn shl() -> rlua::Result<()> {
    #[metamethods(Shl)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Bar,
        Baz
    }

    impl std::ops::Shl for Foo {
        type Output = Self;

        fn shl(self, other: Self) -> Self {
            other
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("bar", Foo::Bar)?;
        globals.set("baz", Foo::Baz)?;

        let res = ctx.load("bar << baz").eval::<Foo>()?;
        assert_eq!(res, Foo::Baz);

        let res = ctx.load("baz << bar").eval::<Foo>()?;
        assert_eq!(res, Foo::Bar);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn shr() -> rlua::Result<()> {
    #[metamethods(Shr)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Bar,
        Baz
    }

    impl std::ops::Shr for Foo {
        type Output = Self;

        fn shr(self, _: Self) -> Self {
            self
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("bar", Foo::Bar)?;
        globals.set("baz", Foo::Baz)?;

        let res = ctx.load("bar >> baz").eval::<Foo>()?;
        assert_eq!(res, Foo::Bar);

        let res = ctx.load("baz >> bar").eval::<Foo>()?;
        assert_eq!(res, Foo::Baz);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn lt() -> rlua::Result<()> {
    #[metamethods(Lt)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Bar(u8),
    }

    impl std::cmp::PartialOrd for Foo {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let self_bar = match self {
                Foo::Bar(bar) => bar
            };
            let other_bar = match other {
                Foo::Bar(bar) => bar
            };
            self_bar.partial_cmp(&other_bar)
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("one", Foo::Bar(1))?;
        globals.set("two", Foo::Bar(2))?;

        let res = ctx.load("one < two").eval::<bool>()?;
        assert!(res);

        let res = ctx.load("two < one").eval::<bool>()?;
        assert!(!res);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn lte() -> rlua::Result<()> {
    #[metamethods(Le)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Foo {
        Bar(u8),
    }

    impl std::cmp::PartialOrd for Foo {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let self_bar = match self {
                Foo::Bar(bar) => bar
            };
            let other_bar = match other {
                Foo::Bar(bar) => bar
            };
            self_bar.partial_cmp(&other_bar)
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("one", Foo::Bar(1))?;
        globals.set("two", Foo::Bar(2))?;
        globals.set("two_again", Foo::Bar(2))?;

        let res = ctx.load("one <= two").eval::<bool>()?;
        assert!(res);

        let res = ctx.load("two <= one").eval::<bool>()?;
        assert!(!res);

        let res = ctx.load("two <= two_again").eval::<bool>()?;
        assert!(res);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn unm() -> rlua::Result<()> {
    #[metamethods(Unm)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Sign {
        Plus,
        Minus,
    }

    impl std::ops::Neg for Sign {
        type Output = Self;
        
        fn neg(self) -> Self {
            match self {
                Sign::Plus => Sign::Minus,
                Sign::Minus => Sign::Plus,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("plus", Sign::Plus)?;
        globals.set("minus", Sign::Minus)?;

        let res = ctx.load("-plus").eval::<Sign>()?;
        assert_eq!(res, Sign::Minus);
        let res = ctx.load("-minus").eval::<Sign>()?;
        assert_eq!(res, Sign::Plus);

        Ok(())
    })?;
    Ok(())
}


#[test]
fn bnot() -> rlua::Result<()> {
    #[metamethods(BNot)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    enum Bool {
        True,
        False,
    }

    impl std::ops::Not for Bool {
        type Output = Self;
        
        fn not(self) -> Self {
            match self {
                Bool::True => Bool::False,
                Bool::False => Bool::True,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("t", Bool::True)?;
        globals.set("f", Bool::False)?;

        let res = ctx.load("~t").eval::<Bool>()?;
        assert_eq!(res, Bool::False);
        let res = ctx.load("~f").eval::<Bool>()?;
        assert_eq!(res, Bool::True);

        Ok(())
    })?;
    Ok(())
}
