((globalThis) => {
	const core = Deno.core;
	const ops = core.ops;

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

	Object.defineProperty(String.prototype, 'parseBytes', {
		value(decimals = 2) {
			if (!+this) return '0B';
			const c = 0 > decimals ? 0 : decimals,
				d = Math.floor(Math.log(this) / Math.log(1024));
			return `${parseFloat((this / Math.pow(1024, d)).toFixed(c))}${['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'][d]}`;
		},
	});

	globalThis.__dirname = ops.op_dirname();
	globalThis.sleep = (ms) => {
		ops.op_sleep(ms);
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

	globalThis.core = {
		readFile: (path) => {
			return core.opAsync('op_read_file', path);
		},
		writeFile: (path, contents) => {
			return core.opAsync('op_write_file', path, contents);
		},
		removeFile: (path) => {
			return ops.op_remove_file(path);
		},
	};

	globalThis.os = {
		release: () => {
			return ops.op_release();
		},
		platform: () => {
			return ops.op_platform();
		},
		freemem: () => {
			return ops.op_freemem();
		},
		totalmem: () => {
			return ops.op_totalmem();
		},
		exit: (code) => {
			return ops.op_exit(code);
		},
	};

	globalThis.process = {
		env: {
			get: (value) => {
				return ops.op_env_get(value);
			},
			toObject: () => {
				return ops.op_env_object();
			},
		},
	};
})(globalThis);
