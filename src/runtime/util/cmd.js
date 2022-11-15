((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.cmd = {
		exec: (cmd) => ops.op_exec(cmd),
		spawn: async (cmd) => core.opAsync('op_spawn', cmd),
	};
})(globalThis);
