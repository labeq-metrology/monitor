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

export const useConnectionStore = defineStore('connections', () => {
    const connections = reactive<Map<string, LabeqConnection>>(new Map());

    invoke<AppState>('get_state').then(state => {
        state.connections.forEach(connection => connections.set(connection.id, connection));
    });

    listen<AppState>('app-state-update', (e) => {
        const ids = new Set(e.payload.workspaces.map(w => w.id));
        for (const key of connections.keys()) {
            if (!ids.has(key)) connections.delete(key);
        }
        e.payload.connections.forEach(connection => connections.set(connection.id, connection));
    })

    const refreshDeviceList = async (connId: string) => {
        await invoke('update_devices', {connId});
    };

    const activateConnection = async (connId: string) => {
        await invoke('connection_activate', {connId});
    };

    const deactivateConnection = async (connId: string) => {
        await invoke('connection_deactivate', {connId});
    };


    return {
        connections,
        refreshDeviceList,
        activateConnection,
        deactivateConnection
    };
});
