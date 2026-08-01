import * as VueRuntime from "vue";
import {createApp} from "vue";
import App from "./App.vue";
import "@/index.css";
import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";
import NewConnectionView from "@/views/NewConnectionView.vue";
import WelcomeView from "@/views/WelcomeView.vue";
import * as PiniaRuntime from "pinia";
import {createPinia} from "pinia";
import {registerEventListeners} from "@/lib/eventHandler.ts";
import {useConnectionStore} from "@/store/connection.store.ts";

(window as any).Vue = VueRuntime;
(window as any).Pinia = PiniaRuntime;

const routes: RouteRecordRaw[] = [
    {path: "/", component: WelcomeView},
    {path: "/new-connection", component: NewConnectionView},
    {
        path: "/connection/:connectionId",
        name: "connection",
        component: () => import("@/views/ConnectionView.vue"),
        beforeEnter: (to) => {
            const connections = useConnectionStore();

            if (!connections.connections.has(to.params.connectionId as string)) {
                console.error("Connection not found");
                return false;
            }

            return true;
        }
    },
    {
        path: "/connection/:connectionId/devices/:id",
        name: "device",
        component: () => import("@/views/DeviceView.vue"),
        beforeEnter: (to) => {
            const connections = useConnectionStore();
            const device = connections.connections
                .get(to.params.connectionId as string)?.devices
                .find(d => d.uuid === to.params.id);

            if (!device) {
                console.error("Device not found");
                return false;
            }

            return true;
        }
    },
    {
        path: "/connection/:connectionId/devices/:id/terminal",
        name: "device-terminal",
        component: () => import("@/views/DeviceTerminalView.vue"),
        beforeEnter: (to) => {
            const connections = useConnectionStore();
            const device = connections.connections
                .get(to.params.connectionId as string)?.devices
                .find(d => d.uuid === to.params.id);

            if (!device) {
                console.error("Device not found");
                return false;
            }

            return true;
        }
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

const app = createApp(App);
const pinia = createPinia();

registerEventListeners();

app.use(router).use(pinia);

app.mount('#app');

