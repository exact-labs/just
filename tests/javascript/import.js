import { test } from 'https://raw.githubusercontent.com/exact-rs/just/master/tests/packages/test/0.0.1/test.js';
import json from '../files/demo.json' assert { type: 'json' };
import { add, multiply } from 'https://x.nest.land/ramda@0.27.0/source/index.js';

const { test: test2 } = await require('test');

console.log(json);
console.log(json.hello);
console.log(json.array);

console.log(add(multiply(2, 5), 5));

test();
test2();
