const { spawn, exec } = cmd;

spawn('rustc --version').then((output) => {
	core.print(output);
});

core.print(exec('rustc --version'));
