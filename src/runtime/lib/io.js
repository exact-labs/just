class EncodingError extends Error {
	constructor(message) {
		super(message);
		this.name = 'EncodingError';
	}
}

const parseFileInfo = (response) => {
	const unix = Just.build.os === 'darwin' || Just.build.os === 'linux';
	return {
		isFile: response.isFile,
		isDirectory: response.isDirectory,
		isSymlink: response.isSymlink,
		size: response.size,
		mtime: response.mtimeSet !== null ? new Date(response.mtime) : null,
		atime: response.atimeSet !== null ? new Date(response.atime) : null,
		birthtime: response.birthtimeSet !== null ? new Date(response.birthtime) : null,
		dev: unix ? response.dev : null,
		ino: unix ? response.ino : null,
		mode: unix ? response.mode : null,
		nlink: unix ? response.nlink : null,
		uid: unix ? response.uid : null,
		gid: unix ? response.gid : null,
		rdev: unix ? response.rdev : null,
		blksize: unix ? response.blksize : null,
		blocks: unix ? response.blocks : null,
	};
};

const file_stat = async (path) => {
	const res = await Just.fn.async('file_stat', {
		path: path,
		lstat: true,
	});
	return parseFileInfo(res);
};

const read_file = async (path, encoding) => {
	return Just.fn.async('read_file', path).then((contents) => {
		switch (encoding.toLowerCase()) {
			case 'utf-8':
			case 'utf8':
				return contents.from_bytes();
			case 'bytes':
			case 'u8':
				return contents;
			default:
				throw new EncodingError('please supply proper encoding format');
		}
	});
};

const write_file = async (path, contents, encoding) => {
	switch (encoding.toLowerCase()) {
		case 'utf-8':
		case 'utf8':
			return Just.fn.async('write_file', path, contents.to_bytes());
		case 'bytes':
		case 'u8':
			return Just.fn.async('write_file', path, contents);
		default:
			throw new EncodingError('please supply proper encoding format');
	}
};

const fs = {
	file: {
		read: (path, encoding = '') => read_file(path, encoding),
		write: (path, contents, encoding = '') => write_file(path, contents, encoding),
		remove: (path) => Just.fn.async('remove_file', path),
		sha: (path) => Just.fn.file_sha(path),
		exists: (path) => Just.fn.file_exists(path),
	},
	dir: {
		read: (path = './') => Just.fn.dir_list(path),
		create: (path) => Just.fn.async('make_dir', path),
		remove: (path) => Just.fn.async('remove_dir', path),
		exists: (path) => Just.fn.dir_exists(path),
	},
	stat: (path) => file_stat(path),
	chmod: (path, mode) => Just.fn.chmod(path, mode),
};

export { fs };
