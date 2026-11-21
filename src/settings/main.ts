import * as VueRuntime from "vue";
import * as PiniaRuntime from "pinia";
import {createApp} from "vue";
import SettingsApp from "@/settings/SettingsApp.vue";
import {createMemoryHistory, createRouter} from "vue-router";
import {createPinia} from "pinia";

import "@/index.css"
import 'vue-sonner/style.css'
import {applyActiveTheme, getStoredAppearanceMode, initColorPalette} from "@/lib/theme";

(window as any).Vue = VueRuntime;
(window as any).Pinia = PiniaRuntime;

const app = createApp(SettingsApp);

applyActiveTheme(initColorPalette(), getStoredAppearanceMode());

const router = createRouter({
    history: createMemoryHistory(),
    routes: [
        {path: "/", redirect: "/appearance"},
        {name: "appearance", path: "/appearance", component: () => import("@/settings/views/AppearanceView.vue")},
        {name: "plugins", path: "/plugins", component: () => import("@/settings/views/PluginsView.vue")}
    ]
});

app.use(router).use(createPinia());
app.mount("#app");