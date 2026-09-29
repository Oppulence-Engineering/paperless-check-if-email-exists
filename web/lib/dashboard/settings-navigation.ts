import {
	AddressBook,
	Buildings,
	ChartLineUp,
	GearSix,
	Key,
	Palette,
	Plugs,
	Question,
	ShieldCheck,
	SlidersHorizontal,
	UserCircle,
	type Icon,
} from "@/lib/icons";

export const settingsGroups = [
	{ key: "workspace", label: "Workspace" },
	{ key: "access", label: "People & access" },
	{ key: "data", label: "Data & connections" },
	{ key: "personal", label: "Personal" },
	{ key: "support", label: "Support" },
] as const;

export const settingsSections: {
	key: string;
	label: string;
	description: string;
	group: (typeof settingsGroups)[number]["key"] | null;
	icon: Icon;
}[] = [
	{
		key: "overview",
		label: "Settings",
		description: "Everything that shapes your workspace and account.",
		group: null,
		icon: GearSix,
	},
	{
		key: "workspace",
		label: "Workspace",
		description: "Name, slug, and active workspace context.",
		group: "workspace",
		icon: Buildings,
	},
	{
		key: "verification",
		label: "Verification",
		description: "Default check policy and result retention.",
		group: "workspace",
		icon: SlidersHorizontal,
	},
	{
		key: "usage",
		label: "Usage",
		description: "Current plan, allowance, and reset date.",
		group: "workspace",
		icon: ChartLineUp,
	},
	{
		key: "branding",
		label: "Branding",
		description: "Workspace identity, colors, and support links.",
		group: "workspace",
		icon: Palette,
	},
	{
		key: "members",
		label: "Members",
		description: "Invitations, membership, and roles.",
		group: "access",
		icon: AddressBook,
	},
	{
		key: "security",
		label: "Authentication & sessions",
		description: "Passkeys, TOTP, and active sessions.",
		group: "access",
		icon: ShieldCheck,
	},
	{
		key: "compliance",
		label: "Security & compliance",
		description: "Administrator policy, SSO, SCIM, and audit history.",
		group: "access",
		icon: Key,
	},
	{
		key: "webhooks",
		label: "Webhooks",
		description: "Pipeline delivery endpoint and signing secret.",
		group: "data",
		icon: Plugs,
	},
	{
		key: "developer",
		label: "API keys",
		description: "Create scoped keys and review recent use.",
		group: "data",
		icon: Key,
	},
	{
		key: "account",
		label: "Account",
		description: "Personal identity and access to this workspace.",
		group: "personal",
		icon: UserCircle,
	},
	{
		key: "help",
		label: "Help",
		description: "Contact support and find product updates.",
		group: "support",
		icon: Question,
	},
];

export function settingsSectionFromParam(value: string | null | undefined) {
	return settingsSections.find((section) => section.key === value) ?? settingsSections[0];
}

export function settingsHref(key: string) {
	return key === "overview" ? "/app/settings" : `/app/settings?settings=${key}`;
}
