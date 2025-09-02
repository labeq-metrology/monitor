export interface LabeqDevice {
    id: string;
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