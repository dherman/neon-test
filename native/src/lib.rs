#[macro_use]
extern crate neon;
extern crate neon_sys;

use std::ops::Drop;

use neon::mem::Handle;
use neon::vm::Lock;
use neon::vm::{JsResult, Call};
use neon::js::{JsFunction, JsString, Value, JsValue, Object, JsObject, JsInteger, JsNumber, JsBoolean, JsUndefined};
use neon::js::class::{JsClass, Class};
use neon::js::binary::JsBuffer;
use neon::js::error::{Kind, JsError};

pub struct Greeter {
    greeting: String
}

impl Drop for Greeter {
    fn drop(&mut self) {
        println!("dropping greeter: {}", self.greeting);
    }
}

declare_types! {

    pub class JsDontCallMe as DontCallMe for () {
        init(_) {
            Ok(())
        }

        method snarf(call) {
            println!("snarf.");
            Ok(JsInteger::new(call.scope, 42).upcast())
        }
    }

    /// A simple native class for creating greeting strings.
    pub class JsGreeter for Greeter {
        init(call) {
            let scope = call.scope;
            println!("extracting the greeting parameter");
            let greeting = try!(try!(call.arguments.require(scope, 0)).to_string(scope)).value();
            println!("extracted the greeting parameter");
            Ok(Greeter {
                greeting: greeting
            })
        }

        call(call) {
            println!("in construct.[[Call]]");
            Ok(JsInteger::new(call.scope, 3).upcast())
        }

        constructor(call) {
            let kind = call.kind();
            let scope = call.scope;
            let greeting = call.arguments.this(scope).grab(|greeter| {
                greeter.greeting.clone()
            });
            println!("in constructor.[[Construct]], call kind is {:?}, greeting is {}", kind, greeting);
            Ok(None)
        }

        method hello(call) {
            let scope = call.scope;
            let name = try!(try!(call.arguments.require(scope, 0)).to_string(scope)).value();
            let msg = call.arguments.this(scope).grab(|greeter| {
                format!("{}, {}!", greeter.greeting, name)
            });
            Ok(try!(JsString::new_or_throw(scope, &msg[..])).upcast())
        }

        method ƒoo(call) {
            Ok(try!(JsString::new_or_throw(call.scope, "scooby dooby doo")).upcast())
        }
    }

}

fn yo(call: Call) -> JsResult<JsObject> {
    Ok(call.arguments.this(call.scope))
}

fn is_greeter(call: Call) -> JsResult<JsBoolean> {
    let x = try!(call.arguments.require(call.scope, 0));
    Ok(JsBoolean::new(call.scope, x.is_a::<JsGreeter>()))
}

fn snorg(call: Call) -> JsResult<JsBoolean> {
    let scope = call.scope;
    let mut b = try!(try!(call.arguments.require(scope, 0)).check::<JsBuffer>());
    Ok(JsBoolean::new(scope, b.grab(|buffer| {
        buffer.as_ref()[0] == 42
    })))
}

fn is_error(call: Call) -> JsResult<JsBoolean> {
    let scope = call.scope;
    let v = try!(call.arguments.require(scope, 0));
    Ok(JsBoolean::new(scope, v.is_a::<JsError>()))
}

fn throw_type_error(_: Call) -> JsResult<JsUndefined> {
    try!(JsError::throw(Kind::TypeError, "ooga booga"));
    Ok(JsUndefined::new())
}

fn is_object(call: Call) -> JsResult<JsBoolean> {
    let scope = call.scope;
    let v = try!(call.arguments.require(scope, 0));
    Ok(JsBoolean::new(scope, v.is_a::<JsObject>()))
}

fn print_integer(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let i = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>());
    println!("integer: {}", i.value());
    Ok(JsUndefined::new())
}

fn call_function(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let f = try!(try!(call.arguments.require(scope, 0)).check::<JsFunction>());
    let args: Vec<Handle<JsValue>> = vec![];
    try!(f.call(scope, JsUndefined::new(), args));
    Ok(JsUndefined::new())
}

fn new_function(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    let f = try!(try!(call.arguments.require(scope, 0)).check::<JsFunction>());
    let zero = JsNumber::new(scope, 0.0);
    let o = try!(f.construct(scope, vec![zero]));
    let get_utc_full_year_method = try!(try!(o.get(scope, "getUTCFullYear")).check::<JsFunction>());
    let args: Vec<Handle<JsValue>> = vec![];
    try!(get_utc_full_year_method.call(scope, o.upcast::<JsValue>(), args)).check::<JsNumber>()
}

register_module!(m, {
    try!(m.export("yo", yo));
    try!(m.export("is_error", is_error));
    try!(m.export("is_greeter", is_greeter));
    try!(m.export("is_object", is_object));
    try!(m.export("print_integer", print_integer));
    try!(m.export("throw_type_error", throw_type_error));
    try!(m.export("call_function", call_function));
    try!(m.export("new_function", new_function));
    let class: Handle<JsClass<JsGreeter>> = try!(JsGreeter::class(m.scope));
    let constructor: Handle<JsFunction<JsGreeter>> = try!(class.constructor(m.scope));
    try!(m.exports.set("Greeter", constructor));
    let f: Handle<JsFunction> = try!(JsFunction::new(m.scope, yo));
    try!(m.exports.set("yoyo", f));
    let class2: Handle<JsClass<JsDontCallMe>> = try!(JsDontCallMe::class(m.scope));
    let constructor: Handle<JsFunction<JsDontCallMe>> = try!(class2.constructor(m.scope));
    try!(m.exports.set("DontCallMe", constructor));
    try!(m.export("snorg", snorg));
    Ok(())
});
