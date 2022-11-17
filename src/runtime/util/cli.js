((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

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
		clear: () => {
			ops.op_stdout('\033[2J\033[1;1H');
		},
	};
})(globalThis);
