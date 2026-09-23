import { ImageResponse } from "next/og";

export const alt = "Oppulence — the commitment ledger that survives kickoff";
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
				<span
					style={{
						alignItems: "center",
						background: "#f97316",
						borderRadius: 16,
						display: "flex",
						fontSize: 28,
						height: 56,
						justifyContent: "center",
						marginRight: 20,
						width: 56,
					}}
				>
					O
				</span>
				Oppulence
			</div>
			<div style={{ display: "flex", flexDirection: "column", maxWidth: 980 }}>
				<div style={{ color: "#a1a1aa", display: "flex", fontSize: 24, marginBottom: 24 }}>
					THE COMMITMENT LEDGER
				</div>
				<div style={{ display: "flex", fontSize: 72, fontWeight: 700, lineHeight: 1.08 }}>
					Business promises, with the proof behind them.
				</div>
			</div>
			<div style={{ color: "#d4d4d8", display: "flex", fontSize: 24 }}>
				Evidence in. Human approval. Accountable action.
			</div>
		</div>,
		size,
	);
}
