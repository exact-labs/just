const parseBody = (p) => {
	if (Array.isArray(p)) return JSON.stringify(p);
	else if (typeof p == 'string') return p;
	else if (p != null && typeof p == 'object') return JSON.stringify(p);
	else return p;
};

const http = {
	get: (url, headers = { 'User-Agent': 'JustRuntime/' + Just.fn.runtime_version() }) => Just.fn.async('op_get', url, JSON.stringify(headers)),
	post: (url, body = '', headers = { 'User-Agent': 'JustRuntime/' + Just.fn.runtime_version() }) =>
		Just.fn.async('op_post', url, parseBody(body), JSON.stringify(headers)),
};

const server = {
	static: (port = 8080, path) => Just.fn.op_static(port, path),
};

export { server, http };
