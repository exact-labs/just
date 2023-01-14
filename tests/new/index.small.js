import { os, cmd } from 'just/sys';

console.fmt.hex.print('#D9A7F7', 'command_exec: ');
console.log(cmd.exec('rustc --version').trim());
