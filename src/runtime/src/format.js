Object.defineProperties(String.prototype, {
	parse_memory: {
		writable: false,
		value: function (decimals = 2) {
			if (!+this) return '0B';
			const c = 0 > decimals ? 0 : decimals,
				d = Math.floor(Math.log(this) / Math.log(1024));
			return `${parseFloat((this / Math.pow(1024, d)).toFixed(c))}${['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'][d]}`;
		},
	},
	to_bytes: {
		writable: false,
		value: function () {
			return ops.to_bytes(String(this));
		},
	},
	json: {
		writable: false,
		value: function () {
			return JSON.parse(this);
		},
	},
	reverse: {
		writable: false,
		value: function () {
			return this.split('').reverse().join('');
		},
	},
	format: {
		writable: false,
		value: function () {
			let args = Array.prototype.slice.call(arguments);
			let i = 0;
			return (output = this.replace(/%s|%d|%f|%@/g, function (match, idx) {
				return (subst = args.slice(0, args.length).slice(i, ++i));
			}));
		},
	},
});

Object.defineProperties(Array.prototype, {
	from_bytes: {
		writable: false,
		value: function () {
			return ops.from_bytes(this);
		},
	},
});

Object.defineProperties(Object.prototype, {
	pretty: {
		writable: false,
		value: function (space = 3, replacer = null) {
			return JSON.stringify(this, replacer, space);
		},
	},
});
