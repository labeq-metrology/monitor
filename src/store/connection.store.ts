import {defineStore} from "pinia";
import {reactive} from "vue";
import {AppState, LabeqConnection} from "@/lib/labeq.ts";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

export interface MqttConnectionEvent {
    id: string;
    url: string;
    event: {
        api_url: string;
        username: string;
        password: string;
    };
}

// TODO: Move all state to Tauri in order to keep track between views
export const useConnectionStore = defineStore('connections', () => {
    const connections = reactive<Map<string, LabeqConnection>>(new Map());

    invoke<AppState>('get_state').then(state => {
        state.connections.forEach(connection => connections.set(connection.id, connection));
    });

    listen<AppState>('app-state-update', (e) => {
        e.payload.connections.forEach(connection => connections.set(connection.id, connection));
        console.log(connections.values())
    }).then(_ => {});

    const refreshDeviceList = async (connection: LabeqConnection) => {
        await invoke('update_devices', {connId: connection.id});
    };

    return {connections, refreshDeviceList};
});
