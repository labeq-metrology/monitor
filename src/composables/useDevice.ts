import {useConnectionStore} from "@/store/connection.store.ts";
import {useRoute} from "vue-router";
import {LabeqDevice} from "@/lib/labeq.ts";
import {ref} from "vue";

export default () => {
    const device = ref<LabeqDevice | undefined>(undefined);
    const connections = useConnectionStore();
    const route = useRoute();
    device.value = connections.connections
        .get(route.params.connectionId as string)?.devices
        .find(d => d.uuid === route.params.id);
    return device;
}