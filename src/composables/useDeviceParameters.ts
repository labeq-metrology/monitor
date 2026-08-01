import {computed, ComputedRef, reactive, Ref, ref} from "vue";
import {LabeqDevice, ParameterInfo, ReadRequestResponse, UpdateMessage} from "@/lib/labeq.ts";
import {useRoute} from "vue-router";
import {LayoutSchema} from "@/lib/plugin.ts";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";

export default (device: ComputedRef<LabeqDevice | undefined>, layout: Ref<LayoutSchema | null>) => {
    const route = useRoute();

    const autoupdate = ref(true); // TODO: Handle autoupdate from rust's side

    const parameters = reactive<Record<string, ParameterInfo>>(device.value?.parameters ?? {});
    const errors = reactive<Record<string, string>>({});
    const activeReadRequests = reactive<Map<string, string>>(new Map());
    const activeWriteRequests = reactive<Map<string, string>>(new Map());

    const refreshing = computed(() => activeReadRequests.size > 0);

    const handleRequest = (match: RegExpExecArray, e: any) => {
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

        if (parsedResponse.error) {
            errors[parsedResponse.parameter] = parsedResponse.error;
            return;
        }

        parameters[parsedResponse.parameter] = parsedResponse.resolvedValue!;
        delete errors[parsedResponse.parameter];
    };

    const handleUpdate = (match: RegExpExecArray, e: any) => {
        if (!autoupdate.value || match.groups?.action !== "update") return;
        const parsedResponse = JSON.parse(
            new TextDecoder().decode(new Uint8Array(e.payload.event.payload))
        ) as UpdateMessage;

        if (parsedResponse.error) {
            errors[match.groups!.parameter] = parsedResponse.error;
            return;
        }

        parameters[match.groups!.parameter] = parsedResponse.newValue!;
        delete errors[match.groups!.parameter];
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
                        connId: route.params.connectionId as string,
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
        const uuid = crypto.randomUUID();
        activeReadRequests.set(uuid, parameter);
        invoke("gateway_publish", {
            connId: route.params.connectionId as string,
            topic: `device/${device.value!.uuid}/parameters/${parameter}/read`,
            payload: JSON.stringify({
                requestId: uuid,
                returnType: "same_as_input",
            }),
            retain: false,
        });
    };

    const writeParameter = async (parameter: string) => {
        console.log("hg")
        if (!device.value) return;
        console.log("hg1")
        const uuid = crypto.randomUUID();
        const res = await invoke("gateway_publish", {
            connId: route.params.connectionId as string,
            topic: `device/${device.value.uuid}/parameters/${parameter}/write`,
            payload: JSON.stringify({
                requestId: uuid,
                contentType: "labeq/string",
                value: parameters[parameter],
            }),
            retain: false,
        });
        console.log(res)
        activeWriteRequests.set(uuid, parameter);
    };

    const writeAllParameters = async () => {
        console.log("ghhg")
        if (!layout.value) return;
        layout.value.rows.forEach(row =>
            row.columns.forEach(column =>
                column.panels.forEach(panel => writeParameter(panel.parameter))
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
        refreshParameters,
        readParameter,
        writeParameter,
        writeAllParameters,
        publishListener,
    };
}
