((globalThis) => {
	Object.defineProperty(String.prototype, 'parseBytes', {
		value(decimals = 2) {
			if (!+this) return '0B';
			const c = 0 > decimals ? 0 : decimals,
				d = Math.floor(Math.log(this) / Math.log(1024));
			return `${parseFloat((this / Math.pow(1024, d)).toFixed(c))}${['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'][d]}`;
		},
	});

	Object.defineProperty(String.prototype, 'json', {
		value() {
			return JSON.parse(this);
		},
	});

	Object.defineProperty(String.prototype, 'reverse', {
		value() {
			return this.split('').reverse().join('');
		},
	});

	String.prototype.format = function () {
		let args = Array.prototype.slice.call(arguments);
		let i = 0;
		return (output = this.replace(/%s|%d|%f|%@/g, function (match, idx) {
			return (subst = args.slice(0, args.length).slice(i, ++i));
		}));
	};

	globalThis.string = {};
})(globalThis);
