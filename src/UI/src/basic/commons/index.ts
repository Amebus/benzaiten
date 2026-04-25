export function isFunction(value: any): value is (...args: never[]) => unknown {
	if (typeof value !== 'function') return false;
	// Exclude classes by checking the function string representation.
	// Regular functions don't start with 'class', but ES6 classes do.
	return !value.toString().startsWith('class ');
}

export function isString(value: any): value is string {
	return typeof value == 'string';
}