import {invoke} from "@tauri-apps/api/core";
import {LabeqPluginManifest} from "@/lib/plugin.ts";
import {LabeqDevice} from "@/lib/labeq.ts";
import {ref} from "vue";

export default (device: LabeqDevice) => {
    const plugin = ref<LabeqPluginManifest | null>(null);
    invoke<string[]>('list_plugins').then(data => {
        plugin.value = data.map(d => JSON.parse(d)).find(plugin => plugin.drivers.includes(device.driver));
    });
    return plugin;
}