((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	const fmt = (...args) => {
		return args
			.map((arg) =>
				JSON.stringify(arg)
					.replace(/\\n/g, '\n')
					.replace(/\\'/g, "'")
					.replace(/\\"/g, '"')
					.replace(/\\&/g, '&')
					.replace(/\\r/g, '\r')
					.replace(/\\t/g, '\t')
					.replace(/\\b/g, '\b')
					.replace(/\\f/g, '\f')
					.slice(1, arg.length + 1)
			)
			.join(' ');
	};

	globalThis.console = {
		log: (...args) => {
			ops.op_stdout(fmt(...args));
		},
		info: (...args) => {
			ops.op_info(fmt(...args));
		},
		error: (...args) => {
			ops.op_stderr(fmt(...args));
		},
		clear: () => {
			ops.op_stdout('\033[2J\033[1;1H');
		},
	};
})(globalThis);
