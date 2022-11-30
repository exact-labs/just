import { test } from 'https://raw.githubusercontent.com/exact-rs/just/master/tests/packages/test/index.js';
import { test as test2 } from './packages/test/index.js';
import json from './files/demo.json' assert { type: 'json' };

console.log(json);
console.log(json.hello);
console.log(json.array);

test();
test2();
