const hex = {
	encode: (str) => Just.fn.hex_encode(str),
	decode: (str) => Just.fn.hex_decode(str),
};

const base64 = {
	encode: (str) => Just.fn.base64_encode(str),
	decode: (str) => Just.fn.base64_decode(str),
	test: (str) => /^([0-9a-zA-Z+/]{4})*(([0-9a-zA-Z+/]{2}==)|([0-9a-zA-Z+/]{3}=))?$/.test(str),
};

export { base64, hex };
