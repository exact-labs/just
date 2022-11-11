((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.runtime = {
		version: () => ops.op_version(),
	};
})(globalThis);
