var addon = require('../native');

var Greeter = addon.Greeter;
var greeter = new Greeter("Hello");
console.log(greeter.hello("dear Neon user"));
try {
  console.log(Greeter("as call"));
} catch(e) {
  console.log("message: " + e.message);
}

var DontCallMe = addon.DontCallMe;
var thing = new DontCallMe();
console.log(thing.snarf());
try {
  console.log(DontCallMe());
} catch(e) {
  console.log("message: " + e.message);
}

var yoyo = addon.yoyo;

console.log(addon.yoyo() === addon);
console.log(yoyo() === global);
console.log(addon.is_greeter(greeter));
console.log(!addon.is_greeter({}));

try {
  Greeter.prototype.hello.call({});
} catch (e) {
  console.log("error: " + e.message);
}

if (global.gc) {
  var object = { greeter: new Greeter("temp") };
  object.greeter = null;
  global.gc();
}

try {
  new Greeter();
} catch (e) {
  console.log("error: " + e.message);
}

console.log(addon.snorg(new Uint8Array([17])));
console.log(addon.snorg(new Uint8Array([42])));

console.log(addon.is_error(new Error('hi')));
console.log(addon.is_error(new TypeError('hi')));
console.log(!addon.is_error({}));
console.log(!addon.is_error([]));

function CustomError(message, extra) {
  Error.captureStackTrace(this, this.constructor);
  Error.call(this, message);
  this.name = this.constructor.name;
  this.message = message;
  this.extra = extra;
}

require('util').inherits(CustomError, Error);

console.log(addon.is_error(new CustomError('blargetyblarg', {})));
console.log((new CustomError('blarg')).message);

try {
  addon.throw_type_error();
} catch (e) {
  console.log(e instanceof TypeError);
  console.log(e.constructor === TypeError);
}

console.log(addon.is_object({}));
console.log(addon.is_object(null));

addon.call_function(function() {
  console.log("zomg I am being called from Rust");
});

console.log(addon.new_function(Date));
