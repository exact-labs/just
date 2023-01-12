cmd.spawn('rustc --version').then((output) => {
	core.print(output);
});

core.print(cmd.exec('rustc --version'));
