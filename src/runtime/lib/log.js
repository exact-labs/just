((globalThis) => {
	globalThis.console = {
		format: format.system,
		color: format.font,
		bg: format.bg,
		log: (...args) => {
			ops.log_stdout(logWithoutObject(...args));
		},
		json: (string, format) => {
			ops.log_stdout(formatChain(string, format));
		},
		info: (...args) => {
			ops.log_info(logWithoutObject(...args));
		},
		error: (...args) => {
			ops.log_stderr(logWithoutObject(...args));
		},
		fmt: {
			print: (color, string) => {
				ops.print(color);
				ops.print(logWithoutObject(string));
				ops.print(format.system.reset);
			},
			println: (color, string) => {
				ops.print(color);
				ops.log_stdout(logWithoutObject(string));
				ops.print(format.system.reset);
			},
			rgb: {
				print: (r = 255, g = 255, b = 255, string) => {
					ops.print(`\x1b[38;2;${r};${g};${b}m`);
					ops.print(logWithoutObject(string));
					ops.print(format.system.reset);
				},
				println: (r = 255, g = 255, b = 255, string) => {
					ops.print(`\x1b[38;2;${r};${g};${b}m`);
					ops.log_stdout(logWithoutObject(string));
					ops.print(format.system.reset);
				},
			},
			hex: {
				print: (hex = '#ffffff', string) => {
					ops.print(`\x1b[38;2;${convertHex(hex)[0].toString()};${convertHex(hex)[1].toString()};${convertHex(hex)[2].toString()}m`);
					ops.print(logWithoutObject(string));
					ops.print(format.system.reset);
				},
				println: (hex = '#ffffff', string) => {
					ops.print(`\x1b[38;2;${convertHex(hex)[0].toString()};${convertHex(hex)[1].toString()};${convertHex(hex)[2].toString()}m`);
					ops.log_stdout(logWithoutObject(string));
					ops.print(format.system.reset);
				},
			},
		},
		clear: () => {
			ops.log_stdout(format.system.clear);
		},
	};
})(globalThis);
