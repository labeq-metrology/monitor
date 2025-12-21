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
    connected: boolean;
    devices: LabeqDevice[];
}

export interface ReadRequestResponse {
    request: string;
    parameter: string;
    resolvedValue?: string;
    error?: string;
}
