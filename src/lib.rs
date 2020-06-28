//! rudeboy - Rlua User Data Extension library
//!
//! Provides derive macros and impl block attribute macros which
//! allow for easily generating an index metamethod and exposing rust methods to
//! lua using the `rlua` crate.
//!
//! # Usage
//! There are five major use cases allowed by the crate, covered in the below
//! sections.
//!
//! ## Index metamethod only
//! This allows the fields of an instance of the UserData struct to be accessed
//! from lua using the `instance.field` syntax, but does not generate or allow
//! the user to add any further methods
//!
//! ```
//!# use rlua::Result;
//!# fn test() -> Result<()> {
//! use rudeboy::IndexSealed;
//!
//! #[derive(IndexSealed)]
//! struct Foo {
//!     bar: String,
//!     baz: f64,
//! }
//!
//! let lua = rlua::Lua::new();
//! lua.context(|ctx| {
//!     // Add an instance of Foo to the lua environment
//!     let globals = ctx.globals();
//!     let bar = "bar".to_string();
//!     let baz = 23.0;
//!     globals.set("a_foo", Foo { bar: bar.clone(), baz })?;
//!
//!     // Use the index metamethod to access fields
//!     let lua_bar = ctx.load("a_foo.bar").eval::<String>()?;
//!     assert_eq!(lua_bar, bar);
//!     let lua_baz = ctx.load("a_foo.baz").eval::<f64>()?;
//!     assert_eq!(lua_baz, baz);
//!
//!     Ok(())
//! })?;
//!# Ok(())
//!# }
//!# let res = test();
//!# println!("{:?}", res);
//!# assert!(res.is_ok());
//! ```
//!
//! ## Index metamethod with additional user definitions
//! This allows the fields of an instance of the UserData struct to be accessed
//! from lua using the `instance.field` syntax, but does not generate an impl
//! for `rlua::UserData`. The user must use the [`RudeboyIndex`] trait to add the
//! index metamethod, but is also free to add additional methods to be accessed
//! from lua
//!
//! ```
//!# use rlua::Result;
//!# fn test() -> Result<()> {
//! use rudeboy::Index;
//!
//! #[derive(Index)]
//! struct Foo {
//!     bar: String,
//!     baz: f64,
//! }
//!
//! impl rlua::UserData for Foo {
//!     fn add_methods<'lua, M: ::rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
//!         // Use the rudeboy-generated trait to add the index metamethod
//!         use rudeboy::RudeboyIndex;
//!         Foo::generate_index(methods);
//!
//!         // Add additional user-defined methods
//!         methods.add_method("user_method", |_, data, ()| {
//!             Ok(data.baz * 2.0)
//!         });
//!     }
//! }
//!
//! let lua = rlua::Lua::new();
//! lua.context(|ctx| {
//!     // Add an instance of Foo to the lua environment
//!     let globals = ctx.globals();
//!     let bar = "bar".to_string();
//!     let baz = 23.0;
//!     globals.set("a_foo", Foo { bar: bar.clone(), baz })?;
//!
//!     // Use the index metamethod to access fields
//!     let lua_bar = ctx.load("a_foo.bar").eval::<String>()?;
//!     assert_eq!(lua_bar, bar);
//!     let lua_baz = ctx.load("a_foo.baz").eval::<f64>()?;
//!     assert_eq!(lua_baz, baz);
//!
//!     // Use the user defined method
//!     let udm = ctx.load("a_foo:user_method()").eval::<f64>()?;
//!     assert_eq!(baz * 2.0, udm);
//!
//!     Ok(())
//! })?;
//!# Ok(())
//!# }
//!# let res = test();
//!# println!("{:?}", res);
//!# assert!(res.is_ok());
//! ```
//!
//! ## Index metamethod and rust methods only
//! This generates an index metamethod and exposes the methods in the tagged impl
//! block to lua, as well as generating an impl of UserData. The user cannot add
//! any further user defined methods, however.
//!
//! ```
//!# use rlua::Result;
//!# fn test() -> Result<()> {
//! use rudeboy::{Index, MethodsSealed};
//!
//! #[derive(Index)]
//! struct Foo {
//!     bar: String,
//!     baz: f64,
//! }
//!
//! #[MethodsSealed]
//! impl Foo {
//!     // Methods must take self as their receiver...
//!     fn double(&self) -> f64 {
//!         self.baz * 2.0
//!     }
//!
//!     // ... but can take mut self as well
//!     fn set_baz(&mut self, baz: f64) {
//!         self.baz = baz;
//!     }
//! }
//!
//! let lua = rlua::Lua::new();
//! lua.context(|ctx| {
//!     // Add an instance of Foo to the lua environment
//!     let globals = ctx.globals();
//!     let bar = "bar".to_string();
//!     let baz = 23.0;
//!     globals.set("a_foo", Foo { bar: bar.clone(), baz })?;
//!
//!     // Use the index metamethod to access fields
//!     let lua_bar = ctx.load("a_foo.bar").eval::<String>()?;
//!     assert_eq!(lua_bar, bar);
//!     let lua_baz = ctx.load("a_foo.baz").eval::<f64>()?;
//!     assert_eq!(lua_baz, baz);
//!
//!     // Use the immutable method
//!     let doubled = ctx.load("a_foo:double()").eval::<f64>()?;
//!     assert_eq!(baz * 2.0, doubled);
//!
//!     // Use the mutable method
//!     ctx.load("a_foo:set_baz(5.0)").exec()?;
//!     let new_baz = ctx.load("a_foo.baz").eval::<f64>()?;
//!     assert_eq!(new_baz, 5.0);
//!
//!     Ok(())
//! })?;
//!# Ok(())
//!# }
//!# let res = test();
//!# println!("{:?}", res);
//!# assert!(res.is_ok());
//! ```
//!
//! ## Index metamethod and rust methods with additional user definitions
//! This generates an index metamethod and exposes the methods in the tagged impl
//! block to lua, but does not generate an impl of UserData. The user can then
//! add additional methods using `rlua::UserData`, but must use the
//! [`RudeboyIndex`] and [`RudeboyMethods`] traits to add the generated methods
//! to the user data.
//!
//! ```
//!# use rlua::Result;
//!# fn test() -> Result<()> {
//! use rudeboy::{Index, Methods};
//!
//! #[derive(Index)]
//! struct Foo {
//!     bar: String,
//!     baz: f64,
//! }
//!
//! #[Methods]
//! impl Foo {
//!     // Methods must take self as their receiver...
//!     fn double(&self) -> f64 {
//!         self.baz * 2.0
//!     }
//!
//!     // ... but can take mut self as well
//!     fn set_baz(&mut self, baz: f64) {
//!         self.baz = baz;
//!     }
//! }
//!
//! impl rlua::UserData for Foo {
//!     fn add_methods<'lua, M: ::rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
//!         // Use the rudeboy-generated trait to add the index metamethod
//!         use rudeboy::RudeboyIndex;
//!         Foo::generate_index(methods);
//!
//!         // Use the rudeboy-generated trait to add the methods from the tagged
//!         // impl block
//!         use rudeboy::RudeboyMethods;
//!         Foo::generate_methods(methods);
//!
//!         // Add additional user-defined methods
//!         methods.add_method("user_method", |_, data, ()| {
//!             Ok(data.baz * 2.0)
//!         });
//!     }
//! }
//!
//! let lua = rlua::Lua::new();
//! lua.context(|ctx| {
//!     // Add an instance of Foo to the lua environment
//!     let globals = ctx.globals();
//!     let bar = "bar".to_string();
//!     let baz = 23.0;
//!     globals.set("a_foo", Foo { bar: bar.clone(), baz })?;
//!
//!     // Use the index metamethod to access fields
//!     let lua_bar = ctx.load("a_foo.bar").eval::<String>()?;
//!     assert_eq!(lua_bar, bar);
//!     let lua_baz = ctx.load("a_foo.baz").eval::<f64>()?;
//!     assert_eq!(lua_baz, baz);
//!
//!     // Use the immutable method
//!     let udm = ctx.load("a_foo:double()").eval::<f64>()?;
//!     assert_eq!(baz * 2.0, udm);
//!
//!     // Use the mutable method
//!     ctx.load("a_foo:set_baz(5.0)").exec()?;
//!     let new_baz = ctx.load("a_foo.baz").eval::<f64>()?;
//!     assert_eq!(new_baz, 5.0);
//!
//!     // Use the user defined method
//!     let udm = ctx.load("a_foo:user_method()").eval::<f64>()?;
//!     assert_eq!(new_baz * 2.0, udm);
//!
//!     Ok(())
//! })?;
//!# Ok(())
//!# }
//!# let res = test();
//!# println!("{:?}", res);
//!# assert!(res.is_ok());
//! ```
//!
//! ## Rust methods with no index metamethod
//! Exposes methods in an impl block to lua without creating an index metamethod
//! for the type. Also generates an impl for `rlua::UserData`, which means the
//! user cannot add additional user-defined methods
//!
//! This example uses [`MethodsSealed`] to add methods from an impl block as well
//! as generating a `rlua::UserData` impl for the type. Note that
//! [`MethodsSealed`] expects an implementation of [`RudeboyIndex`] for the type,
//! so we must use the derive macro [`NoIndex`] in order to provide an empty
//! implementation of that trait.
//!
//! ```
//!# use rlua::Result;
//!# fn test() -> Result<()> {
//! use rudeboy::{NoIndex, MethodsSealed};
//!
//! // Derives Clone so that an instance can be retrieved from the lua context
//! #[derive(Clone, NoIndex)]
//! struct Foo {
//!     bar: String,
//!     baz: f64,
//! }
//!
//! #[MethodsSealed]
//! impl Foo {
//!     // Methods must take self as their receiver...
//!     fn double(&self) -> f64 {
//!         self.baz * 2.0
//!     }
//!
//!     // ... but can take mut self as well
//!     fn set_baz(&mut self, baz: f64) {
//!         self.baz = baz;
//!     }
//! }
//!
//! let lua = rlua::Lua::new();
//! lua.context(|ctx| {
//!     // Add an instance of Foo to the lua environment
//!     let globals = ctx.globals();
//!     let bar = "bar".to_string();
//!     let baz = 23.0;
//!     globals.set("a_foo", Foo { bar: bar.clone(), baz })?;
//!
//!     // Use the immutable method
//!     let udm = ctx.load("a_foo:double()").eval::<f64>()?;
//!     assert_eq!(baz * 2.0, udm);
//!
//!     // Use the mutable method
//!     ctx.load("a_foo:set_baz(5.0)").exec()?;
//!     let new_foo = ctx.load("a_foo").eval::<Foo>()?;
//!     assert_eq!(new_foo.baz, 5.0);
//!
//!     Ok(())
//! })?;
//!# Ok(())
//!# }
//!# let res = test();
//!# println!("{:?}", res);
//!# assert!(res.is_ok());
//! ``` 
//!
//! ## Rust methods with no index metamethod, but with user defined methods
//! This generates an impl for [`RudeboyMethods`] that will add the methods from
//! an impl block, but will not generate an index metamethod or generate an impl
//! for `rlua::UserData`. This allows the user to add additional user-defined
//! methods.
//!
//! Note that because this approach does not use [`MethodsSealed`], there is no
//! need for the struct to use the [`NoIndex`] derive macro.
//!
//! ```
//!# use rlua::Result;
//!# fn test() -> Result<()> {
//! use rudeboy::{Index, Methods};
//!
//! // Derives Clone so that an instance can be retrieved from the lua context
//! #[derive(Clone)]
//! struct Foo {
//!     bar: String,
//!     baz: f64,
//! }
//!
//! #[Methods]
//! impl Foo {
//!     // Methods must take self as their receiver...
//!     fn double(&self) -> f64 {
//!         self.baz * 2.0
//!     }
//!
//!     // ... but can take mut self as well
//!     fn set_baz(&mut self, baz: f64) {
//!         self.baz = baz;
//!     }
//! }
//!
//! impl rlua::UserData for Foo {
//!     fn add_methods<'lua, M: ::rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
//!         // Note: the call for RudeboyIndex::generate_index is simply ommitted
//!
//!         // Use the rudeboy-generated trait to add the methods from the tagged
//!         // impl block
//!         use rudeboy::RudeboyMethods;
//!         Foo::generate_methods(methods);
//!
//!         // Add additional user-defined methods
//!         methods.add_method("user_method", |_, data, ()| {
//!             Ok(data.baz * 2.0)
//!         });
//!     }
//! }
//!
//! let lua = rlua::Lua::new();
//! lua.context(|ctx| {
//!     // Add an instance of Foo to the lua environment
//!     let globals = ctx.globals();
//!     let bar = "bar".to_string();
//!     let baz = 23.0;
//!     globals.set("a_foo", Foo { bar: bar.clone(), baz })?;
//!
//!     // Use the immutable method
//!     let udm = ctx.load("a_foo:double()").eval::<f64>()?;
//!     assert_eq!(baz * 2.0, udm);
//!
//!     // Use the mutable method
//!     ctx.load("a_foo:set_baz(5.0)").exec()?;
//!     let new_foo = ctx.load("a_foo").eval::<Foo>()?;
//!     assert_eq!(new_foo.baz, 5.0);
//!
//!     // Use the user defined method
//!     let udm = ctx.load("a_foo:user_method()").eval::<f64>()?;
//!     assert_eq!(new_foo.baz * 2.0, udm);
//!
//!     Ok(())
//! })?;
//!# Ok(())
//!# }
//!# let res = test();
//!# println!("{:?}", res);
//!# assert!(res.is_ok());
//! ```
//!
//! [`RudeboyIndex`]: trait.RudeboyIndex.html
//! [`RudeboyMethods`]: trait.RudeboyMethods.html
//! [`NoIndex`]: derive.NoIndex.html
//! [`MethodsSealed`]: attr.MethodsSealed.html
pub use rudeboy_derive::{Index, IndexSealed, NoIndex, Methods, MethodsSealed};

use rlua::{UserData, UserDataMethods};

/// Used to generate an index metamethod for a UserData struct
///
/// Implementations provided by [`Index`], [`NoIndex`], and [`IndexSealed`]
///
/// [`Index`]: derive.Index.html
/// [`NoIndex`]: derive.NoIndex.html
/// [`IndexSealed`]: derive.IndexSealed.html
pub trait RudeboyIndex : Sized + UserData {
    fn generate_index<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M);
}

/// Used to expose, to rlua, rust methods for a UserData struct
///
/// Implementations provided by [`Methods`] and [`MethodsSealed`]
///
/// [`Methods`]: attr.Methods.html
/// [`MethodsSealed`]: attr.MethodsSealed.html
pub trait RudeboyMethods : Sized + UserData {
    fn generate_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M);
}
