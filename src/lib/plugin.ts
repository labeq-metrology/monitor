// TODO: Move to a separate package
export interface LabeqPluginManifest {
    name: string,
    version: string,
    id: string,
    drivers: string[],
    panels?: PanelPluginManifest[],
    author?: string,
    description?: string,
}

export interface PanelPluginManifest {
    id: string,
    execute?: boolean,
    bundle?: string,
    style?: string,
}

export interface LayoutSchema {
    rows: LayoutRow[];
}

export interface LayoutRow {
    columns: LayoutColumn[];
    groupName?: string;
}

export interface LayoutPanel {
    kind: string;
    parameter: string;
    displayName?: string;
    readonly: boolean;
    writeOnly?: boolean;
    default: any;
}

export interface LayoutColumn {
    panels: LayoutPanel[];
}