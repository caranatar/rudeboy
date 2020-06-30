use rlua::Lua;
use rudeboy::{
    methods,
    user_data
};

#[test]
fn no_params() -> rlua::Result<()> {
    #[user_data(Methods)]
    enum Foo {
        Single(u32),
        Double(u32),
    }

    #[methods]
    impl Foo {
        pub fn get(&self) -> u32 {
            match self {
                Foo::Single(x) => *x,
                Foo::Double(x) => x * 2,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("single", Foo::Single(5))?;
        globals.set("double", Foo::Double(5))?;

        let res = ctx.load("single:get()").eval::<u32>()?;
        assert_eq!(res, 5);
        let res = ctx.load("double:get()").eval::<u32>()?;
        assert_eq!(res, 10);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn one_builtin() -> rlua::Result<()> {
    #[user_data(Methods)]
    enum Foo {
        Single(u32),
        Double(u32),
    }

    #[methods]
    impl Foo {
        pub fn get_add(&self, other: u32) -> u32 {
            match self {
                Foo::Single(x) => x + other,
                Foo::Double(x) => x * 2 + other,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("single", Foo::Single(5))?;
        globals.set("double", Foo::Double(5))?;

        let res = ctx.load("single:get_add(5)").eval::<u32>()?;
        assert_eq!(res, 10);
        let res = ctx.load("double:get_add(10)").eval::<u32>()?;
        assert_eq!(res, 20);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn one_userdata() -> rlua::Result<()> {
    #[user_data(Methods)]
    enum Foo {
        Single,
        Double,
    }

    #[user_data]
    #[derive(Clone)]
    struct Bar {
        x: u32,
    }

    #[methods]
    impl Foo {
        pub fn get(&self, other: Bar) -> u32 {
            match self {
                Foo::Single => other.x,
                Foo::Double => other.x * 2,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("single", Foo::Single)?;
        globals.set("double", Foo::Double)?;
        globals.set("bar", Bar { x: 5 })?;

        let res = ctx.load("single:get(bar)").eval::<u32>()?;
        assert_eq!(res, 5);
        let res = ctx.load("double:get(bar)").eval::<u32>()?;
        assert_eq!(res, 10);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn multi_builtin() -> rlua::Result<()> {
    #[user_data(Methods)]
    enum Foo {
        Single(u32),
        Double(u32),
    }

    #[methods]
    impl Foo {
        pub fn get_add(&self, a: u32, b: u32) -> u32 {
            match self {
                Foo::Single(x) => x + a + b,
                Foo::Double(x) => x * 2 + a + b,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("single", Foo::Single(5))?;
        globals.set("double", Foo::Double(5))?;

        let res = ctx.load("single:get_add(5, 5)").eval::<u32>()?;
        assert_eq!(res, 15);
        let res = ctx.load("double:get_add(10, 10)").eval::<u32>()?;
        assert_eq!(res, 30);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn multi_userdata() -> rlua::Result<()> {
    #[user_data(Methods)]
    enum Foo {
        Single,
        Double,
    }

    #[user_data]
    #[derive(Clone)]
    struct Bar {
        x: u32,
    }

    #[methods]
    impl Foo {
        pub fn get(&self, a: Bar, b: Bar) -> u32 {
            match self {
                Foo::Single => a.x + b.x,
                Foo::Double => (a.x + b.x) * 2,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("single", Foo::Single)?;
        globals.set("double", Foo::Double)?;
        globals.set("bar_one", Bar { x: 5 })?;
        globals.set("bar_two", Bar { x: 4 })?;

        let res = ctx.load("single:get(bar_one, bar_two)").eval::<u32>()?;
        assert_eq!(res, 9);
        let res = ctx.load("double:get(bar_one, bar_two)").eval::<u32>()?;
        assert_eq!(res, 18);

        Ok(())
    })?;

    Ok(())
}

#[test]
fn multi_mix() -> rlua::Result<()> {
    #[user_data(Methods)]
    enum Foo {
        Single,
        Double,
    }

    #[user_data]
    #[derive(Clone)]
    struct Bar {
        x: u32,
    }

    #[methods]
    impl Foo {
        pub fn get(&self, a: Bar, b: u32) -> u32 {
            match self {
                Foo::Single => a.x + b,
                Foo::Double => (a.x + b) * 2,
            }
        }
    }

    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        globals.set("single", Foo::Single)?;
        globals.set("double", Foo::Double)?;
        globals.set("bar_one", Bar { x: 5 })?;

        let res = ctx.load("single:get(bar_one, 4)").eval::<u32>()?;
        assert_eq!(res, 9);
        let res = ctx.load("double:get(bar_one, 4)").eval::<u32>()?;
        assert_eq!(res, 18);

        Ok(())
    })?;

    Ok(())
}
