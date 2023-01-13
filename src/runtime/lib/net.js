const http = {
	get: (url, headers = { 'User-Agent': 'JustRuntime/' + Just.fn.runtime_version() }) => Just.fn.async('net_get', url, JSON.stringify(headers)),
	post: (url, body = '', headers = { 'User-Agent': 'JustRuntime/' + Just.fn.runtime_version() }) =>
		Just.fn.async('net_post', url, parseObject(body), JSON.stringify(headers)),
};

const server = {
	static: (path, host = '0.0.0.0', port = 3000) => Just.fn.async('serve_directory', host, port, path),
	string: (string, type = 'text/plain; charset=UTF-8', host = '0.0.0.0', port = 3000) => Just.fn.async('serve_string', host, port, string, type),
};

export { server, http };
