export function clamp(t) {
	return t < 0 ? 0 : t > 1 ? 1 : t;
}
export function lerp(a, b, t) {
	return (1 - t) * a + b * t;
}
export function lerpClamp(a, b, t) {
	const q = (1 - t) * a + b * t;
	return t < 0 ? a : t > 1 ? b : q;
}
export function invLerp(a, b, value) {
	return (value - a) / (b - a);
}
export function remap(iMin, iMax, oMin, oMax, v) {
	return lerp(oMin, oMax, invLerp(iMin, iMax, v));
}
export function easeOut(x) {
	return 1 - Math.pow(1 - x, 3);
}
export function easeInQuart(x) {
	return x * 2;
}
export function easeOutExpo(x) {
	return x === 1 ? 1 : 1 - Math.pow(2, -10 * x);
}
export function easeInExpo(x) {
	return x === 0 ? 0 : Math.pow(2, 10 * x - 10);
}
