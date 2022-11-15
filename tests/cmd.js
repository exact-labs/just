cmd.spawn('rustc --version').then((output) => {
	console.log(output);
});

console.log(cmd.exec('rustc --version'));
