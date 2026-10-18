/** Each icon module default-exports its SVG data; the package types only the barrel. */
declare module "@hugeicons/core-free-icons/*" {
	const icon: import("@hugeicons/react").IconSvgElement;
	export default icon;
}
