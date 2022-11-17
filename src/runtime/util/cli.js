((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	const format = {
		system: {
			reset: '\x1b[0m',
			bold: '\x1b[1m',
			dim: '\x1b[2m',
			italic: '\x1b[3m',
			underscore: '\x1b[4m',
			reverse: '\x1b[7m',
			strikethrough: '\x1b[9m',
			backoneline: '\x1b[1A',
			cleanthisline: '\x1b[K',
			clear: '\033[2J',
		},
		font: {
			black: '\x1b[30m',
			red: '\x1b[31m',
			green: '\x1b[32m',
			yellow: '\x1b[33m',
			blue: '\x1b[34m',
			magenta: '\x1b[35m',
			cyan: '\x1b[36m',
			white: '\x1b[37m',
		},
		bg: {
			black: '\x1b[40m',
			red: '\x1b[41m',
			green: '\x1b[42m',
			yellow: '\x1b[43m',
			blue: '\x1b[44m',
			magenta: '\x1b[45m',
			cyan: '\x1b[46m',
			white: '\x1b[47m',
		},
	};

	const convertHex = (hex) => {
		var c;
		if (/^#([A-Fa-f0-9]{3}){1,2}$/.test(hex)) {
			c = hex.substring(1).split('');
			if (c.length == 3) {
				c = [c[0], c[0], c[1], c[1], c[2], c[2]];
			}
			c = '0x' + c.join('');
			return [(c >> 16) & 255, (c >> 8) & 255, c & 255];
		}
	};

	const formatChain = (string, format = false) =>
		JSON.stringify(string, null, format ? 2 : 0)
			.replace(/\\n/g, '\n')
			.replace(/\\'/g, "'")
			.replace(/\\"/g, '"')
			.replace(/\\&/g, '&')
			.replace(/\\r/g, '\r')
			.replace(/\\t/g, '\t')
			.replace(/\\b/g, '\b')
			.replace(/\\f/g, '\f');

	const logWithoutObject = (...args) => {
		return args.map((arg) => (typeof arg == 'object' ? formatChain(arg) : formatChain(arg.toString()).slice(1, arg.toString().length + 1))).join(' ');
	};

	globalThis.console = {
		format: format.system,
		color: format.font,
		bg: format.bg,
		log: (...args) => {
			ops.op_stdout(logWithoutObject(...args));
		},
		json: (string, format) => {
			ops.op_stdout(formatChain(string, format));
		},
		info: (...args) => {
			ops.op_info(logWithoutObject(...args));
		},
		error: (...args) => {
			ops.op_stderr(logWithoutObject(...args));
		},
		fmt: {
			print: (color, string) => {
				ops.op_print(color);
				ops.op_print(logWithoutObject(string));
				ops.op_print(format.system.reset);
			},
			println: (color, string) => {
				ops.op_print(color);
				ops.op_stdout(logWithoutObject(string));
				ops.op_print(format.system.reset);
			},
			rgb: {
				print: (r = 255, g = 255, b = 255, string) => {
					ops.op_print(`\x1b[38;2;${r};${g};${b}m`);
					ops.op_print(logWithoutObject(string));
					ops.op_print(format.system.reset);
				},
				println: (r = 255, g = 255, b = 255, string) => {
					ops.op_print(`\x1b[38;2;${r};${g};${b}m`);
					ops.op_stdout(logWithoutObject(string));
					ops.op_print(format.system.reset);
				},
			},
			hex: {
				print: (hex = '#ffffff', string) => {
					ops.op_print(`\x1b[38;2;${convertHex(hex)[0].toString()};${convertHex(hex)[1].toString()};${convertHex(hex)[2].toString()}m`);
					ops.op_print(logWithoutObject(string));
					ops.op_print(format.system.reset);
				},
				println: (hex = '#ffffff', string) => {
					ops.op_print(`\x1b[38;2;${convertHex(hex)[0].toString()};${convertHex(hex)[1].toString()};${convertHex(hex)[2].toString()}m`);
					ops.op_stdout(logWithoutObject(string));
					ops.op_print(format.system.reset);
				},
			},
		},
		clear: () => {
			ops.op_stdout(format.system.clear);
		},
	};
})(globalThis);
