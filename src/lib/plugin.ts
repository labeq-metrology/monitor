// TODO: Move to a separate package
export interface LabeqPluginManifest {
    name: string,
    version: string,
    id: string,
    drivers: string,
    author: string,
    description: string,
}

export interface LayoutSchema {
    rows: LayoutRow[];
}

export interface LayoutRow {
    columns: LayoutColumn[];
    groupName?: string;
}

export interface LayoutPanel {
    kind: "text" | "number" | "number-unit" | "chart" | "image" | "custom";
    parameter: string;
    displayName?: string;
    readonly: boolean;
}

export interface LayoutColumn {
    panels: LayoutPanel[];
}