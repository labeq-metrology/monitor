import {defineStore} from "pinia";
import {reactive} from "vue";
import {listen} from "@tauri-apps/api/event";
import {LabeqConnection, LabeqDevice} from "@/lib/labeq.ts";
import {useFetch} from "@vueuse/core";

export interface MqttConnectionEvent {
    id: string;
    url: string;
    event: number;
}

export const useConnectionStore = defineStore('connections', () => {
    const connections = reactive<Map<string, LabeqConnection>>(new Map());

    const refreshDeviceList = async (connection: LabeqConnection) => {
        const {data} = await useFetch(connection.apiUrl + '/devices').json<LabeqDevice[]>().get();
        connection.devices = data.value ?? [];
    }

    listen<MqttConnectionEvent>('mqtt-connected', async (event) => {
        if (!connections.has(event.payload.id)) return;
        const connection = connections.get(event.payload.id)!!;
        connection.connected = true;
        await refreshDeviceList(connection);
    })

    listen<MqttConnectionEvent>('mqtt-disconnected', async (event) => {
        if (!connections.has(event.payload.id)) return;
        connections.get(event.payload.id)!!.connected = false;
    })

    return {connections, refreshDeviceList}
});
