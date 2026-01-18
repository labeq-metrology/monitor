export interface AppState {
    connections: LabeqConnection[];
}

export interface LabeqDevice {
    uuid: string;
    name: string;
    model: string;
    manufacturer: string;
    driver: string;
}

export interface LabeqConnection {
    id: string;
    mqttUrl: string;
    apiUrl: string;
    username: string;
    password: string;
    state: "Disconnected" | "Connecting" | "Connected" | "ConnectionError";
    devices: LabeqDevice[];
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
