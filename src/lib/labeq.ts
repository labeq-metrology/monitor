export interface AppState {
    connections: LabeqConnection[];
    workspaces: LabeqWorkspace[];
}

export type ParameterValue = string | number | number[];

export interface ParameterInfo {
    paramType: string,
    value: ParameterValue
}

export interface LabeqDevice {
    uuid: string;
    name: string;
    model: string;
    manufacturer: string;
    driver: string;
    tags: string[];
    parameters: Record<string, ParameterInfo>
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
    scripts: LabeqScriptStatus[];
}

export interface LabeqScriptStatus {
    executionId: string;
    script: string;
    status: string;
    heartbeatId?: number;
    lastHeartbeatAt?: number;
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
    resolvedValue?: ParameterInfo;
    error?: string;
}

export interface UpdateMessage {
    newValue: ParameterInfo;
    error?: string;
}
