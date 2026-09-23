import contract from "@/config/contracts/backend.openapi.json";

export function GET() {
	return Response.json(contract, {
		headers: { "Cache-Control": "public, max-age=300", "Access-Control-Allow-Origin": "*" },
	});
}
