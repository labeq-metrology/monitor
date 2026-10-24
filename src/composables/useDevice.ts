import {useConnectionStore} from "@/store/connection.store.ts";
import {computed, MaybeRefOrGetter, toValue} from "vue";

export default (connectionId: MaybeRefOrGetter<string>, deviceId: MaybeRefOrGetter<string>) => {
    const connections = useConnectionStore();

    return computed(() =>
        connections.connections
            ?.get(toValue(connectionId))
            ?.devices.find(d => d.uuid === toValue(deviceId))
    );
};
