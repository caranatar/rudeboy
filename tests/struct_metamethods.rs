use rlua::Lua;
use rudeboy::{
    metamethods,
    user_data
};

#[test]
fn index() -> rlua::Result<()> {
    #[metamethods(Index)]
    #[user_data(MetaMethods)]
    struct Person {
        pub name: String,
        pub greeting: String,
        pub number: f64,
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        let name = "Eris".to_string();
        let expected_name = name.clone();
        let greeting = "Hail".to_string();
        let expected_greeting = greeting.clone();
        let number = 23.0;
        globals.set("eris", Person { name, greeting, number, })?;

        let out_name = ctx.load("eris.name").eval::<String>()?;
        assert_eq!(expected_name, out_name);
        let out_greeting = ctx.load("eris.greeting").eval::<String>()?;
        assert_eq!(expected_greeting, out_greeting);
        let out_number = ctx.load("eris.number").eval::<f64>()?;
        assert_eq!(number, out_number);

        let bad_index = ctx.load("eris.bad_index").eval::<f64>();
        assert!(bad_index.is_err());

        Ok(())
    })?;
    Ok(())
}

#[test]
fn nested_index() -> rlua::Result<()> {
    #[metamethods(Index)]
    #[user_data(MetaMethods)]
    #[derive(Clone)]
    struct Bar {
        pub number: f64,
    }
    
    #[metamethods(Index)]
    #[user_data(MetaMethods)]
    #[derive(Clone)]
    struct Foo {
        pub number: f64,
        pub bar: Bar,
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        let bar = Bar { number: 5.0 };
        let foo = Foo { bar, number: 23.0 };
        globals.set("foo", foo)?;

        let foo_num = ctx.load("foo.number").eval::<f64>()?;
        assert_eq!(foo_num, 23.0);
        let bar_num = ctx.load("foo.bar.number").eval::<f64>()?;
        assert_eq!(bar_num, 5.0);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn add() -> rlua::Result<()> {
    #[metamethods(Add)]
    #[user_data(MetaMethods)]
    #[derive(Clone, PartialEq, Eq, Copy, Debug)]
    struct Foo {
        pub bar: i32
    }

    impl std::ops::Add for Foo {
        type Output = Self;
    
        fn add(self, other: Self) -> Self {
            Foo { bar: self.bar + other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 2 })?;
        globals.set("foo_two", Foo { bar: 3 })?;

        let sum = ctx.load("foo_one + foo_two").eval::<Foo>()?;
        assert_eq!(sum.bar, 5);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn sub() -> rlua::Result<()> {
    #[metamethods(Sub)]
    #[user_data(MetaMethods)]
    #[derive(Clone, PartialEq, Eq, Copy, Debug)]
    struct Foo {
        pub bar: i32
    }

    impl std::ops::Sub for Foo {
        type Output = Self;
    
        fn sub(self, other: Self) -> Self {
            Foo { bar: self.bar - other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 5 })?;
        globals.set("foo_two", Foo { bar: 3 })?;

        let diff = ctx.load("foo_one - foo_two").eval::<Foo>()?;
        assert_eq!(diff.bar, 2);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn eq() -> rlua::Result<()> {
    #[metamethods(Eq)]
    #[user_data(MetaMethods)]
    #[derive(PartialEq, Clone, Debug)]
    struct Foo {
        pub bar: f32
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 5.0 })?;
        globals.set("foo_two", Foo { bar: 5.0 })?;
        globals.set("foo_three", Foo { bar: 23.0 })?;

        let cmp = ctx.load("foo_one == foo_two").eval::<bool>()?;
        assert!(cmp);

        let cmp = ctx.load("foo_one == foo_three").eval::<bool>()?;
        assert!(!cmp);

        let cmp = ctx.load("foo_three == foo_three").eval::<bool>()?;
        assert!(cmp);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn mul() -> rlua::Result<()> {
    #[metamethods(Mul)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy)]
    struct Foo {
        pub bar: f32
    }

    impl std::ops::Mul for Foo {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            Foo { bar: self.bar * other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 5.0 })?;
        globals.set("foo_two", Foo { bar: 5.0 })?;

        ctx.load("foo_three = foo_one * foo_two").exec()?;

        let res = ctx.load("foo_three").eval::<Foo>()?;
        assert_eq!(res.bar, 25.0);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn div() -> rlua::Result<()> {
    #[metamethods(Div)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy)]
    struct Foo {
        pub bar: f32
    }

    impl std::ops::Div for Foo {
        type Output = Self;

        fn div(self, other: Self) -> Self {
            Foo { bar: self.bar / other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 25.0 })?;
        globals.set("foo_two", Foo { bar: 5.0 })?;

        ctx.load("foo_three = foo_one / foo_two").exec()?;

        let res = ctx.load("foo_three").eval::<Foo>()?;
        assert_eq!(res.bar, 5.0);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn mod_() -> rlua::Result<()> {
    #[metamethods(Mod)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy)]
    struct Foo {
        pub bar: i32
    }

    impl std::ops::Rem for Foo {
        type Output = Self;

        fn rem(self, other: Self) -> Self {
            Foo { bar: self.bar % other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 23 })?;
        globals.set("foo_two", Foo { bar: 10 })?;

        ctx.load("foo_three = foo_one % foo_two").exec()?;

        let res = ctx.load("foo_three").eval::<Foo>()?;
        assert_eq!(res.bar, 3);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn band() -> rlua::Result<()> {
    #[metamethods(BAnd)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy)]
    struct Foo {
        pub bar: u8
    }

    impl std::ops::BitAnd for Foo {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            Foo { bar: self.bar & other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 0b11110000 })?;
        globals.set("foo_two", Foo { bar: 0b10101010 })?;

        ctx.load("foo_three = foo_one & foo_two").exec()?;

        let res = ctx.load("foo_three").eval::<Foo>()?;
        assert_eq!(res.bar, 0b10100000);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn bor() -> rlua::Result<()> {
    #[metamethods(BOr)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy)]
    struct Foo {
        pub bar: u8
    }

    impl std::ops::BitOr for Foo {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            Foo { bar: self.bar | other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 0b11110000 })?;
        globals.set("foo_two", Foo { bar: 0b10101010 })?;

        ctx.load("foo_three = foo_one | foo_two").exec()?;

        let res = ctx.load("foo_three").eval::<Foo>()?;
        assert_eq!(res.bar, 0b11111010);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn bxor() -> rlua::Result<()> {
    #[metamethods(BXor)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy)]
    struct Foo {
        pub bar: u8
    }

    impl std::ops::BitXor for Foo {
        type Output = Self;

        fn bitxor(self, other: Self) -> Self {
            Foo { bar: self.bar ^ other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 0b11110000 })?;
        globals.set("foo_two", Foo { bar: 0b10101010 })?;

        ctx.load("foo_three = foo_one ~ foo_two").exec()?;

        let res = ctx.load("foo_three").eval::<Foo>()?;
        assert_eq!(res.bar, 0b01011010);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn shl() -> rlua::Result<()> {
    #[metamethods(Shl)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy)]
    struct Foo {
        pub bar: u8
    }

    impl std::ops::Shl for Foo {
        type Output = Self;

        fn shl(self, other: Self) -> Self {
            Foo { bar: self.bar << other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 0b00001010 })?;
        globals.set("foo_two", Foo { bar: 4 })?;

        ctx.load("foo_three = foo_one << foo_two").exec()?;

        let res = ctx.load("foo_three").eval::<Foo>()?;
        assert_eq!(res.bar, 0b10100000);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn shr() -> rlua::Result<()> {
    #[metamethods(Shr)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy)]
    struct Foo {
        pub bar: u8
    }

    impl std::ops::Shr for Foo {
        type Output = Self;

        fn shr(self, other: Self) -> Self {
            Foo { bar: self.bar >> other.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 0b10100000 })?;
        globals.set("foo_two", Foo { bar: 4 })?;

        ctx.load("foo_three = foo_one >> foo_two").exec()?;

        let res = ctx.load("foo_three").eval::<Foo>()?;
        assert_eq!(res.bar, 0b00001010);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn lt() -> rlua::Result<()> {
    #[metamethods(Lt)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    struct Foo {
        pub bar: u8
    }

    impl std::cmp::PartialOrd for Foo {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.bar.partial_cmp(&other.bar)
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 5 })?;
        globals.set("foo_two", Foo { bar: 4 })?;

        let res = ctx.load("foo_one < foo_two").eval::<bool>()?;
        assert!(!res);

        let res = ctx.load("foo_two < foo_one").eval::<bool>()?;
        assert!(res);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn lte() -> rlua::Result<()> {
    #[metamethods(Le)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    struct Foo {
        pub bar: u8
    }

    impl std::cmp::PartialOrd for Foo {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.bar.partial_cmp(&other.bar)
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo_one", Foo { bar: 5 })?;
        globals.set("foo_two", Foo { bar: 4 })?;
        globals.set("foo_three", Foo { bar: 4 })?;

        let res = ctx.load("foo_one <= foo_two").eval::<bool>()?;
        assert!(!res);

        let res = ctx.load("foo_two <= foo_one").eval::<bool>()?;
        assert!(res);

        let res = ctx.load("foo_two <= foo_three").eval::<bool>()?;
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
    struct Foo {
        pub bar: i8
    }

    impl std::ops::Neg for Foo {
        type Output = Self;
        
        fn neg(self) -> Foo {
            Foo { bar: -self.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo", Foo { bar: 5 })?;

        let res = ctx.load("-foo").eval::<Foo>()?;
        assert_eq!(res.bar, -5);

        Ok(())
    })?;
    Ok(())
}

#[test]
fn bnot() -> rlua::Result<()> {
    #[metamethods(BNot)]
    #[user_data(MetaMethods)]
    #[derive(Clone, Debug, Copy, PartialEq)]
    struct Foo {
        pub bar: u8
    }

    impl std::ops::Not for Foo {
        type Output = Self;
        
        fn not(self) -> Foo {
            Foo { bar: !self.bar }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo", Foo { bar: 0b00001111 })?;

        let res = ctx.load("~foo").eval::<Foo>()?;
        assert_eq!(res.bar, 0b11110000);

        Ok(())
    })?;
    Ok(())
}
