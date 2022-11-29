((globalThis) => {
	Deno.core.initializeAsyncOps();
	globalThis.runtime = {
		version: () => Deno.core.ops.op_version(),
		internal: Deno,
	};
})(globalThis);
