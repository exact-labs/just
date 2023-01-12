const fs = {
	read: {
		file: (path) => Just.fn.async('read_file', path),
		dir: (path = './') => Just.fn.dir_list(path),
	},
	write: {
		file: (path, contents) => Just.fn.async('write_file', path, contents),
		dir: (path) => Just.fn.async('make_dir', path),
	},
	remove: {
		file: (path) => Just.fn.async('remove_file', path),
		dir: (path) => Just.fn.async('remove_dir', path),
	},
	sha: (path) => Just.fn.file_sha(path),
};

export { fs };
