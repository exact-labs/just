import { env } from 'just/sys';

console.log(process.env.DATA);
console.log(env.get('DATA'));
env.set('DATA', 'test');
console.log(process.env.DATA);
