export interface AppState {
    connections: LabeqConnection[];
    workspaces: LabeqWorkspace[];
}

export interface LabeqDevice {
    uuid: string;
    name: string;
    model: string;
    manufacturer: string;
    driver: string;
    tags: string[];
}

export interface LabeqConnection {
    id: string;
    name: string;
    mqttUrl: string;
    apiUrl: string;
    username: string;
    password: string;
    state: "Disconnected" | "Connecting" | "Connected" | "ConnectionError";
    devices: LabeqDevice[];
}

export interface Folder<T> {
    items: Item<T>[];
}

export type Item<T> =
    | { type: 'item', value: T }
    | { type: 'folder', value: NestedFolder<T> };

export interface NestedFolder<T> {
    id: string;
    name: string;
    items: Item<T>[];
}

export interface LabeqWorkspace {
    id: string;
    name: string;
    connections: Folder<string>;
    devices: Folder<string>;
}

export interface ReadRequestResponse {
    request: string;
    parameter: string;
    resolvedValue?: string;
    error?: string;
}

export interface UpdateMessage {
    newValue: string;
    error?: string;
}
