import {createApp} from "vue";
import SettingsApp from "@/settings/SettingsApp.vue";
import {createMemoryHistory, createRouter} from "vue-router";
import WelcomeView from "@/views/WelcomeView.vue";

import "@/index.css"
import 'vue-sonner/style.css'

const app = createApp(SettingsApp);

const router = createRouter({
    history: createMemoryHistory(),
    routes: [
        {name: "popup-home", path: "/popup.html", component: WelcomeView},
        {name: "plugins", path: "/plugins", component: () => import("@/settings/views/PluginsView.vue")}
    ]
})

app.use(router);
app.mount("#app");