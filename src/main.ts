import {createApp} from "vue";
import App from "./App.vue";
import "@/index.css";
import {createMemoryHistory, createRouter, RouteRecordRaw} from "vue-router";
import NewConnectionView from "@/views/NewConnectionView.vue";
import WelcomeView from "@/views/WelcomeView.vue";
import {createPinia} from "pinia";
import {registerEventListeners} from "@/lib/eventHandler.ts";

const routes: RouteRecordRaw[] = [
    {path: "/", component: WelcomeView},
    {path: "/new-connection", component: NewConnectionView}
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

const app = createApp(App);
const pinia = createPinia();

registerEventListeners();

app.use(router).use(pinia);

app.mount('#app');

