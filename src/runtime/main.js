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

	Object.defineProperty(String.prototype, 'parseBytes', {
		value(decimals = 2) {
			if (!+this) return '0B';
			const c = 0 > decimals ? 0 : decimals,
				d = Math.floor(Math.log(this) / Math.log(1024));
			return `${parseFloat((this / Math.pow(1024, d)).toFixed(c))}${['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'][d]}`;
		},
	});

	globalThis.__modules = ops.op_packages_dir();
	globalThis.__dirname = ops.op_dirname();
	globalThis.sleep = (ms) => ops.op_sleep(ms);

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

	globalThis.fs = {
		readFile: (path) => core.opAsync('op_read_file', path),
		writeFile: (path, contents) => core.opAsync('op_write_file', path, contents),
		removeFile: (path) => ops.op_remove_file(path),
	};

	globalThis.os = {
		release: () => ops.op_release(),
		platform: () => ops.op_platform(),
		machine: () => ops.op_machine(),
		uptime: () => ops.op_uptime(),
		freemem: () => ops.op_freemem(),
		totalmem: () => ops.op_totalmem(),
		loadavg: () => ops.op_loadavg(),
		exit: (code) => ops.op_exit(code),
	};

	globalThis.process = {
		env: {
			args: () => ops.op_env(),
			get: (value) => ops.op_env_get(value),
			set: (key, value) => ops.op_env_set(key, value),
		},
		cwd: () => ops.op_dirname(),
	};

	globalThis.core = {
		encode: (text) => ops.op_encode(text),
		encode_fast: (text) => ops.op_encode_fast(text),
		escape: (text) => ops.op_escape(text),
	};
})(globalThis);
