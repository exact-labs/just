((globalThis) => {
	globalThis.process = {
		env: {
			get: (value) => ops.op_env_get(value),
			set: (key, value) => ops.op_env_set(key, value),
		},
		cwd: () => ops.op_dirname(),
	};
})(globalThis);
