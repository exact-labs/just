((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.http = {
		get: (url) => core.opAsync('op_get', url),
		post: (url, body, json = false) => core.opAsync('op_post', url, json ? JSON.stringify(body) : body),
	};

	globalThis.server = {
		static: (port = 8080, path) => ops.op_static(port, path),
	};
})(globalThis);
