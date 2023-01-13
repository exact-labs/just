const format = {
	system: {
		reset: '\x1b[0m',
		bold: '\x1b[1m',
		dim: '\x1b[2m',
		italic: '\x1b[3m',
		underline: '\x1b[4m',
		reverse: '\x1b[7m',
		strike: '\x1b[9m',
		backline: '\x1b[1A',
		cleanline: '\x1b[K',
		clear: '\033[2J',
	},
	font: {
		black: '\x1b[30m',
		red: '\x1b[31m',
		green: '\x1b[32m',
		yellow: '\x1b[33m',
		blue: '\x1b[34m',
		magenta: '\x1b[35m',
		cyan: '\x1b[36m',
		white: '\x1b[37m',
	},
	bg: {
		black: '\x1b[40m',
		red: '\x1b[41m',
		green: '\x1b[42m',
		yellow: '\x1b[43m',
		blue: '\x1b[44m',
		magenta: '\x1b[45m',
		cyan: '\x1b[46m',
		white: '\x1b[47m',
	},
};

const convertHex = (hex) => {
	var c;
	if (/^#([A-Fa-f0-9]{3}){1,2}$/.test(hex)) {
		c = hex.substring(1).split('');
		if (c.length == 3) {
			c = [c[0], c[0], c[1], c[1], c[2], c[2]];
		}
		c = '0x' + c.join('');
		return [(c >> 16) & 255, (c >> 8) & 255, c & 255];
	}
};

const formatChain = (string, format = false) =>
	JSON.stringify(string, null, format ? 2 : 0)
		.replace(/\\n/g, '\n')
		.replace(/\\'/g, "'")
		.replace(/\\"/g, '"')
		.replace(/\\&/g, '&')
		.replace(/\\r/g, '\r')
		.replace(/\\t/g, '\t')
		.replace(/\\b/g, '\b')
		.replace(/\\f/g, '\f');

const logWithoutObject = (...args) => {
	return args.map((arg) => (typeof arg == 'object' ? formatChain(arg) : formatChain(arg.toString()).slice(1, arg.toString().length + 1))).join(' ');
};

const NEWLINES_MATCH = /\r\n|\n|\r/;
const NEWLINE = '\n';
const RE_INI_KEY_VAL = /^\s*([\w.-]+)\s*=\s*(.*)?\s*$/;
const RE_NEWLINES = /\\n/g;

const parseObject = (p) => {
	if (Array.isArray(p)) return JSON.stringify(p);
	else if (typeof p == 'string') return p;
	else if (p != null && typeof p == 'object') return JSON.stringify(p);
	else return String(p);
};

const parseBuffer = (src) => {
	const obj = {};
	src
		.toString()
		.split(NEWLINES_MATCH)
		.forEach((line, idx) => {
			const keyValueArr = line.match(RE_INI_KEY_VAL);
			if (keyValueArr != null) {
				const key = keyValueArr[1];
				let val = keyValueArr[2] || '';
				const end = val.length - 1;
				const isDoubleQuoted = val[0] === '"' && val[end] === '"';
				const isSingleQuoted = val[0] === "'" && val[end] === "'";

				if (isSingleQuoted || isDoubleQuoted) {
					val = val.substring(1, end);
					if (isDoubleQuoted) {
						val = val.replace(RE_NEWLINES, NEWLINE);
					}
				} else {
					val = val.trim();
				}
				obj[key] = val;
			}
		});
	return obj;
};
