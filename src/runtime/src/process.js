((globalThis) => {
	globalThis.process = {
		env: Just.env_store,
		cwd: ops.op_dirname(),
	};
})(globalThis);
