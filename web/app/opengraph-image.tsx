import { ImageResponse } from "next/og";

export const alt = "Verifier — check email addresses before you send";
export const size = { width: 1200, height: 630 };
export const contentType = "image/png";

/** Static, deterministic social card generated and cached by Next.js. */
export default function OpenGraphImage() {
	return new ImageResponse(
		<div
			style={{
				background: "#0a0a0a",
				color: "#ffffff",
				display: "flex",
				flexDirection: "column",
				height: "100%",
				justifyContent: "space-between",
				padding: "72px 80px",
				width: "100%",
			}}
		>
			<div style={{ alignItems: "center", display: "flex", fontSize: 28, fontWeight: 600 }}>
				<svg height="56" style={{ marginRight: 20 }} viewBox="80 110 480 420" width="64">
					<polygon
						fill="#9E9E9E"
						points="152.0,514.0 544.0,514.0 516.0,465.5 460.0,465.5 348.0,271.5 320.0,320.0 404.0,465.5 180.0,465.5"
					/>
					<polygon
						fill="#4A4A4A"
						points="516.0,465.5 320.0,126.0 292.0,174.5 320.0,223.0 208.0,417.0 264.0,417.0 348.0,271.5 460.0,465.5"
					/>
					<polygon
						fill="#F2F2F2"
						points="292.0,174.5 96.0,514.0 152.0,514.0 180.0,465.5 404.0,465.5 376.0,417.0 208.0,417.0 320.0,223.0"
					/>
				</svg>
				Verifier
			</div>
			<div style={{ display: "flex", flexDirection: "column", maxWidth: 980 }}>
				<div style={{ color: "#a1a1aa", display: "flex", fontSize: 24, marginBottom: 24 }}>
					EMAIL VERIFICATION
				</div>
				<div style={{ display: "flex", fontSize: 72, fontWeight: 700, lineHeight: 1.08 }}>
					Check email addresses before you send.
				</div>
			</div>
			<div style={{ color: "#d4d4d8", display: "flex", fontSize: 24 }}>
				Clean lists. Suppress bounces. Protect your sender reputation.
			</div>
		</div>,
		size,
	);
}
