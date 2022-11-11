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

	Object.defineProperty(String.prototype, 'json', {
		value() {
			return JSON.parse(this);
		},
	});

	globalThis.internal = () => core;
	globalThis.pkg = (file) => ops.op_packages_dir() + `/${file}/index.js`;
	globalThis.__dirname = ops.op_dirname();
	globalThis.sleep = (ms) => ops.op_sleep(ms);

	globalThis.http = {
		get: (url) => core.opAsync('op_get', url),
		post: (url, body, json = false) => core.opAsync('op_post', url, json ? JSON.stringify(body) : body),
	};

	globalThis.server = {
		static: (port = 8080, path) => ops.op_static(port, path),
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

	globalThis.fs = {
		readFile: (path) => core.opAsync('op_read_file', path),
		writeFile: (path, contents) => core.opAsync('op_write_file', path, contents),
		removeFile: (path) => ops.op_remove_file(path),
	};

	globalThis.os = {
		release: () => ops.op_release(),
		platform: () => ops.op_platform(),
		machine: () => ops.op_machine(),
		hostname: () => ops.op_hostname().slice(1, -1),
		homedir: () => ops.op_homedir(),
		cpus: () => ops.op_cpus(),
		uptime: () => ops.op_uptime(),
		freemem: () => ops.op_freemem(),
		totalmem: () => ops.op_totalmem(),
		loadavg: () => ops.op_loadavg(),
		exit: (code) => ops.op_exit(code),
	};

	globalThis.process = {
		env: {
			get: (value) => ops.op_env_get(value),
			set: (key, value) => ops.op_env_set(key, value),
		},
		cwd: () => ops.op_dirname(),
	};

	globalThis.core = {
		print: (text) => ops.op_print(text),
		listen: () => ops.op_listen(),
		encode: (text) => ops.op_encode(text),
		encode_fast: (text) => ops.op_encode_fast(text),
		escape: (text) => ops.op_escape(text),
		id: {
			secure: (len = 21) => ops.op_id(len),
			basic: (rounds = 4) => [...Array(rounds)].map((i) => Math.round(Date.now() + Math.random() * Date.now()).toString(36)).join(''),
			uuid: () => {
				var lut = [];
				var d0 = (Math.random() * 0xffffffff) | 0;
				var d1 = (Math.random() * 0xffffffff) | 0;
				var d2 = (Math.random() * 0xffffffff) | 0;
				var d3 = (Math.random() * 0xffffffff) | 0;
				for (var i = 0; i < 256; i++) {
					lut[i] = (i < 16 ? '0' : '') + i.toString(16);
				}
				return (
					lut[d0 & 0xff] +
					lut[(d0 >> 8) & 0xff] +
					lut[(d0 >> 16) & 0xff] +
					lut[(d0 >> 24) & 0xff] +
					'-' +
					lut[d1 & 0xff] +
					lut[(d1 >> 8) & 0xff] +
					'-' +
					lut[((d1 >> 16) & 0x0f) | 0x40] +
					lut[(d1 >> 24) & 0xff] +
					'-' +
					lut[(d2 & 0x3f) | 0x80] +
					lut[(d2 >> 8) & 0xff] +
					'-' +
					lut[(d2 >> 16) & 0xff] +
					lut[(d2 >> 24) & 0xff] +
					lut[d3 & 0xff] +
					lut[(d3 >> 8) & 0xff] +
					lut[(d3 >> 16) & 0xff] +
					lut[(d3 >> 24) & 0xff]
				);
			},
		},
	};
})(globalThis);
