import {computed, ComputedRef, reactive, Ref, ref, toValue, MaybeRefOrGetter} from "vue";
import {LabeqDevice, ParameterInfo, ReadRequestResponse, UpdateMessage} from "@/lib/labeq.ts";
import {LayoutSchema} from "@/lib/plugin.ts";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";

export default (connectionId: MaybeRefOrGetter<string>, device: ComputedRef<LabeqDevice | undefined>, layout: Ref<LayoutSchema | null>) => {
    const connIdStr = toValue(connectionId);
    const autoupdate = ref(true);

    const parameters = reactive<Record<string, ParameterInfo>>(device.value?.parameters ?? {});
    const errors = reactive<Record<string, string>>({});
    const activeReadRequests = reactive<Map<string, string>>(new Map());
    const activeWriteRequests = reactive<Map<string, string>>(new Map());

    const dirtyParameters = reactive<Record<string, boolean>>({});

    const refreshing = computed(() => activeReadRequests.size > 0);

    const isParameterProcessing = (parameter: string) => computed(() =>
        Array.from(activeReadRequests.values()).includes(parameter) ||
        Array.from(activeWriteRequests.values()).includes(parameter)
    );

    const isParameterRefreshing = (parameter: string) => computed(() =>
        Array.from(activeReadRequests.values()).includes(parameter)
    );

    const handleRequest = (match: RegExpExecArray, e: any) => {
        if (match.groups?.device !== device.value?.uuid) return;

        if (!match.groups || match.groups.action !== "resolve") return;
        if (
            !activeReadRequests.has(match.groups.request) &&
            !activeWriteRequests.has(match.groups.request)
        ) return;

        const parsedResponse = JSON.parse(
            new TextDecoder().decode(new Uint8Array(e.payload.event.payload))
        ) as ReadRequestResponse;

        activeReadRequests.delete(match.groups.request);
        activeWriteRequests.delete(match.groups.request);

        const paramName = parsedResponse.parameter;

        if (parsedResponse.error) {
            errors[paramName] = parsedResponse.error;
            return;
        }

        parameters[paramName] = parsedResponse.resolvedValue!;

        delete errors[paramName];
        delete dirtyParameters[paramName];
    };

    const handleUpdate = (match: RegExpExecArray, e: any) => {
        if (match.groups?.device !== device.value?.uuid) return;

        if (!autoupdate.value || match.groups?.action !== "update") return;
        const parsedResponse = JSON.parse(
            new TextDecoder().decode(new Uint8Array(e.payload.event.payload))
        ) as UpdateMessage;

        const paramName = match.groups!.parameter;

        if (parsedResponse.error) {
            errors[paramName] = parsedResponse.error;
            return;
        }

        parameters[paramName] = parsedResponse.newValue!;

        delete errors[paramName];
        delete dirtyParameters[paramName];
    };

    const publishListener = listen("mqtt-publish", (e: any) => {
        const requestRegex = /^device\/(?<device>.+)\/requests\/(?<request>.+)\/(?<action>.+)/;
        const updateRegex = /^device\/(?<device>.+)\/parameters\/(?<parameter>.+)\/(?<action>.+)/;

        const requestMatch = requestRegex.exec(e.payload.event.topic);
        const updateMatch = updateRegex.exec(e.payload.event.topic);

        if (requestMatch?.groups) handleRequest(requestMatch, e);
        else if (updateMatch?.groups) handleUpdate(updateMatch, e);
    });

    const refreshParameters = async () => {
        if (!layout.value || !device.value) return;
        layout.value.rows.forEach(row =>
            row.columns.forEach(column =>
                column.panels.forEach(panel => {
                    const uuid = crypto.randomUUID();
                    activeReadRequests.set(uuid, panel.parameter);
                    invoke("gateway_publish", {
                        connId: connIdStr,
                        topic: `device/${device.value!.uuid}/parameters/${panel.parameter}/read`,
                        payload: JSON.stringify({
                            requestId: uuid,
                            returnType: "same_as_input",
                        }),
                        retain: false,
                    });
                })
            )
        );
    };

    const readParameter = async (parameter: string) => {
        if (!device.value) return;
        const uuid = crypto.randomUUID();
        activeReadRequests.set(uuid, parameter);
        await invoke("gateway_publish", {
            connId: connIdStr,
            topic: `device/${device.value!.uuid}/parameters/${parameter}/read`,
            payload: JSON.stringify({
                requestId: uuid,
                returnType: "same_as_input",
            }),
            retain: false,
        });
    };

    const writeParameter = async (parameter: string) => {
        if (!device.value) return;
        const uuid = crypto.randomUUID();
        const paramToSend = parameters[parameter];

        await invoke("gateway_publish", {
            connId: connIdStr,
            topic: `device/${device.value.uuid}/parameters/${parameter}/write`,
            payload: JSON.stringify({
                requestId: uuid,
                value: paramToSend,
            }),
            retain: false,
        });

        activeWriteRequests.set(uuid, parameter);
        delete dirtyParameters[parameter];
    };

    const writeAllParameters = async () => {
        if (!layout.value) return;
        layout.value.rows.forEach(row =>
            row.columns.forEach(column =>
                column.panels.forEach(panel => {
                    if (!panel.readonly) writeParameter(panel.parameter);
                })
            )
        );
    };

    return {
        autoupdate,
        parameters,
        errors,
        refreshing,
        activeReadRequests,
        activeWriteRequests,
        dirtyParameters,
        refreshParameters,
        readParameter,
        writeParameter,
        writeAllParameters,
        publishListener,
        isParameterProcessing,
        isParameterRefreshing
    };
}
