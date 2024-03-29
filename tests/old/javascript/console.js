core.print(`${console.format.underline}basic tests${console.format.reset}:\n\n`);

console.log(1);
console.log(1, 1);
console.log(1, 'hello');
console.log('hello', 1);
console.log('hello', 1, 'world');

core.print(`\n${console.format.underline}basic colors${console.format.reset}:\n\n`);

console.fmt.println(console.color.red, 'red');
console.fmt.println(console.color.yellow, 'yellow');
console.fmt.println(console.color.green, 'green');
console.fmt.println(console.color.cyan, 'cyan');
console.fmt.println(console.color.blue, 'blue');
console.fmt.println(console.color.magenta, 'magenta');

core.print(`\n${console.format.underline}inline basic colors${console.format.reset}:\n\n`);

console.fmt.print(console.color.red, 'red ');
console.fmt.print(console.color.yellow, 'yellow ');
console.fmt.print(console.color.green, 'green ');
console.fmt.print(console.color.cyan, 'cyan ');
console.fmt.print(console.color.blue, 'blue ');
console.fmt.println(console.color.magenta, 'magenta ');

core.print(`\n${console.format.underline}rgb colors${console.format.reset}:\n\n`);

console.fmt.rgb.println(0, 0, 0, 'rgb(0, 0, 0)');
console.fmt.rgb.println(255, 0, 0, 'rgb(255, 0, 0)');
console.fmt.rgb.println(0, 255, 0, 'rgb(0, 255, 0)');
console.fmt.rgb.println(0, 0, 255, 'rgb(0, 0, 255)');
console.fmt.rgb.println(255, 255, 0, 'rgb(255, 255, 0)');
console.fmt.rgb.println(0, 255, 255, 'rgb(0, 255, 255)');
console.fmt.rgb.println(255, 0, 255, 'rgb(255, 0, 255)');
console.fmt.rgb.println(255, 255, 255, 'rgb(255, 255, 255)');

core.print(`\n${console.format.underline}inline rgb colors${console.format.reset}:\n\n`);

console.fmt.rgb.print(0, 0, 0, 'rgb(0, 0, 0) ');
console.fmt.rgb.print(255, 0, 0, 'rgb(255, 0, 0) ');
console.fmt.rgb.print(0, 255, 0, 'rgb(0, 255, 0) ');
console.fmt.rgb.print(0, 0, 255, 'rgb(0, 0, 255) ');
console.fmt.rgb.print(255, 255, 0, 'rgb(255, 255, 0) ');
console.fmt.rgb.print(0, 255, 255, 'rgb(0, 255, 255) ');
console.fmt.rgb.print(255, 0, 255, 'rgb(255, 0, 255) ');
console.fmt.rgb.println(255, 255, 255, 'rgb(255, 255, 255) ');

core.print(`\n${console.format.bold}hex colors${console.format.reset}:\n\n`);
core.print(`  ${console.format.underline}pride flag${console.format.reset}:\n\n`);

console.fmt.hex.println('#E50104', '  ███████████████████');
console.fmt.hex.println('#FE8B00', '  ███████████████████');
console.fmt.hex.println('#FEED00', '  ███████████████████');
console.fmt.hex.println('#008026', '  ███████████████████');
console.fmt.hex.println('#004CFF', '  ███████████████████');
console.fmt.hex.println('#750786', '  ███████████████████');

core.print(`\n  ${console.format.underline}bi flag${console.format.reset}:\n\n`);

console.fmt.hex.println('#D70071', '  ███████████████████');
console.fmt.hex.println('#D70071', '  ███████████████████');
console.fmt.hex.println('#9C4E98', '  ███████████████████');
console.fmt.hex.println('#0135AA', '  ███████████████████');
console.fmt.hex.println('#0135AA', '  ███████████████████');

core.print(`\n  ${console.format.underline}trans flag${console.format.reset}:\n\n`);

console.fmt.hex.println('#59D0FA', '  ███████████████████');
console.fmt.hex.println('#F5ABBA', '  ███████████████████');
console.fmt.hex.println('#FFFFFF', '  ███████████████████');
console.fmt.hex.println('#F5ABBA', '  ███████████████████');
console.fmt.hex.println('#59D0FA', '  ███████████████████');
