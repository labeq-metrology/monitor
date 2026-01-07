import {defineStore} from "pinia";
import {reactive} from "vue";
import {listen} from "@tauri-apps/api/event";
import {LabeqConnection, LabeqDevice} from "@/lib/labeq.ts";
import {useFetch} from "@vueuse/core";

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

    const refreshDeviceList = async (connection: LabeqConnection) => {
        const {data} = await useFetch(connection.apiUrl + '/devices').json<LabeqDevice[]>().get();
        connection.devices = data.value ?? [];
    };

    listen<MqttConnectionEvent>('mqtt-connected', async (event) => {
        if (!connections.has(event.payload.id)) {
            connections.set(event.payload.id, {
                id: event.payload.id,
                mqttUrl: event.payload.url,
                apiUrl: event.payload.event.api_url,
                username: event.payload.event.username,
                password: event.payload.event.password,
                connected: false,
                devices: []
            });
        };
        const connection = connections.get(event.payload.id)!!;
        connection.connected = true;
        await refreshDeviceList(connection);
    });

    listen<MqttConnectionEvent>('mqtt-disconnected', async (event) => {
        if (!connections.has(event.payload.id)) return;
        connections.get(event.payload.id)!!.connected = false;
    });

    return {connections, refreshDeviceList};
});
