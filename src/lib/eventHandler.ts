import {listen} from "@tauri-apps/api/event";
import {toast} from "vue-sonner";

interface LabeqErrorEvent {
    message: string;
    code: string;
}

const ERROR_TOAST_DURATION = 5 * 60 * 1000; // 5 minutes

export function registerEventListeners() {
    listenPluginErrors();
}

const listenPluginErrors = () => {
    listen<LabeqErrorEvent>('plugin-error', (e) => {
        console.log(e);
        toast(`Plugin error (${e.payload.code})`, {
            description: e.payload.message,
            duration: ERROR_TOAST_DURATION,
            action: {
                label: "Close",
                onClick: () => {},
            },
        });
    });
};