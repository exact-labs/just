((globalThis) => {
	globalThis.process = {
		env: Just.env_store,
		cwd: ops.os_dirname(),
	};
})(globalThis);
