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

const fs = {
	file: {
		read: (path) => Just.fn.async('read_file', path),
		write: (path, contents) => Just.fn.async('write_file', path, contents),
		remove: (path) => Just.fn.async('remove_file', path),
		sha: (path) => Just.fn.file_sha(path),
	},
	dir: {
		read: (path = './') => Just.fn.dir_list(path),
		create: (path) => Just.fn.async('make_dir', path),
		remove: (path) => Just.fn.async('remove_dir', path),
	},
	stat: (path) => file_stat(path),
};

export { fs };
