import {LabeqDevice} from "@/lib/labeq.ts";
import {DevicePanelProps} from "@/lib/device.ts";
import {computed, inject, Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {ZodType} from "zod";

export default <T>(panel: DevicePanelProps<any>, validator: ZodType) => {
    const config = ref<T | undefined>(undefined);
    const device = inject<Ref<LabeqDevice>>('device');
    console.log(device?.value)
    if (!device) return computed(() => undefined);
    invoke<string>('get_panel_information', {
        driver: device.value.driver,
        variant: device.value.model,
        parameter: panel.parameter
    }).then(data => {
        try {
            const parsedData = JSON.parse(data);
            const validated = validator.safeParse(parsedData);
            if (validated.success) config.value = validated.data as T;
        } catch (e) {
            return computed(() => undefined);
        }
    });
    return config;
}
