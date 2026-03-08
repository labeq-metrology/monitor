import {useConnectionStore} from "@/store/connection.store.ts";
import {useRoute} from "vue-router";
import {computed} from "vue";

export default () => {
    const connections = useConnectionStore();
    const route = useRoute();
    return computed(() => connections.connections?.get(route.params.connectionId as string)?.devices
        .find(d => d.uuid === route.params.id));
}