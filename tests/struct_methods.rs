use rlua::Lua;
use rudeboy::{methods, user_data};

#[test]
fn no_params() -> rlua::Result<()> {
    #[user_data(Methods)]
    struct Foo {
        pub bar: u8,
    }

    #[methods]
    impl Foo {
        pub fn get_bar(&self) -> u8 {
            self.bar
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo", Foo { bar: 23 })?;

        let bar = ctx.load("foo:get_bar()").eval::<u8>()?;
        assert_eq!(bar, 23);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn one_builtin() -> rlua::Result<()> {
    #[user_data(Methods)]
    struct Foo {
        pub bar: u8,
    }

    #[methods]
    impl Foo {
        pub fn get_add(&self, other: u8) -> u8 {
            self.bar + other
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo", Foo { bar: 23 })?;

        let bar = ctx.load("foo:get_add(23)").eval::<u8>()?;
        assert_eq!(bar, 46);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn one_userdata() -> rlua::Result<()> {
    #[user_data(Methods)]
    struct Foo {
        pub bar: u8,
    }

    #[user_data]
    #[derive(Clone)]
    enum Bar {
        Single,
        Double,
    }

    #[methods]
    impl Foo {
        pub fn get(&self, amount: Bar) -> u8 {
            match amount {
                Bar::Single => self.bar,
                Bar::Double => self.bar * 2,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo", Foo { bar: 23 })?;
        globals.set("single", Bar::Single)?;
        globals.set("double", Bar::Double)?;

        let bar = ctx.load("foo:get(single)").eval::<u8>()?;
        assert_eq!(bar, 23);
        let bar = ctx.load("foo:get(double)").eval::<u8>()?;
        assert_eq!(bar, 46);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn multi_builtin() -> rlua::Result<()> {
    #[user_data(Methods)]
    struct Foo {
        pub bar: u8,
    }

    #[methods]
    impl Foo {
        pub fn get_add(&self, a: u8, b: u8) -> u8 {
            self.bar + a + b
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo", Foo { bar: 23 })?;

        let bar = ctx.load("foo:get_add(23, 5)").eval::<u8>()?;
        assert_eq!(bar, 51);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn multi_userdata() -> rlua::Result<()> {
    #[user_data(Methods)]
    struct Foo {
        pub bar: u8,
    }

    #[user_data]
    #[derive(Clone)]
    enum Bar {
        Single,
        Double,
    }

    impl Bar {
        fn apply(&self, x: u8) -> u8 {
            match self {
                Bar::Single => x,
                Bar::Double => x * 2,
            }
        }
    }

    #[methods]
    impl Foo {
        pub fn get(&self, a: Bar, b: Bar) -> u8 {
            a.apply(b.apply(self.bar))
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo", Foo { bar: 23 })?;
        globals.set("single", Bar::Single)?;
        globals.set("double", Bar::Double)?;

        let bar = ctx.load("foo:get(single, single)").eval::<u8>()?;
        assert_eq!(bar, 23);
        let bar = ctx.load("foo:get(double, single)").eval::<u8>()?;
        assert_eq!(bar, 46);
        let bar = ctx.load("foo:get(double, double)").eval::<u8>()?;
        assert_eq!(bar, 92);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn multi_mix() -> rlua::Result<()> {
    #[user_data(Methods)]
    struct Foo {
        pub bar: u8,
    }

    #[user_data]
    #[derive(Clone)]
    enum Bar {
        Single,
        Double,
    }

    impl Bar {
        fn apply(&self, x: u8) -> u8 {
            match self {
                Bar::Single => x,
                Bar::Double => x * 2,
            }
        }
    }

    #[methods]
    impl Foo {
        pub fn get(&self, a: Bar, b: u8) -> u8 {
            a.apply(b + self.bar)
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("foo", Foo { bar: 23 })?;
        globals.set("single", Bar::Single)?;
        globals.set("double", Bar::Double)?;

        let bar = ctx.load("foo:get(single, 5)").eval::<u8>()?;
        assert_eq!(bar, 28);
        let bar = ctx.load("foo:get(double, 0)").eval::<u8>()?;
        assert_eq!(bar, 46);
        let bar = ctx.load("foo:get(double, 7)").eval::<u8>()?;
        assert_eq!(bar, 60);

        Ok(())
    })?;

    Ok(())
}
