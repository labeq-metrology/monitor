import { defineStore } from 'pinia';
import { ref } from 'vue';
import { Router } from 'vue-router';

export interface Tab {
    id: string;
    title: string;
    path: string;
}

export const useTabsStore = defineStore('tabs', () => {
    const tabs = ref<Tab[]>([]);
    const activeTabId = ref<string | null>(null);

    const addTab = (tab: Tab) => {
        if (!tabs.value.find(t => t.id === tab.id)) {
            tabs.value.push(tab);
        }
        activeTabId.value = tab.id;
    };

    const removeTab = (id: string, router: Router) => {
        const index = tabs.value.findIndex(t => t.id === id);
        if (index !== -1) {
            tabs.value.splice(index, 1);
            if (activeTabId.value === id) {
                const nextTab = tabs.value[index] || tabs.value[index - 1];
                if (nextTab) {
                    router.push(nextTab.path);
                } else {
                    router.push('/');
                }
            }
        }
    };

    const reorderTabs = (fromIndex: number, toIndex: number) => {
        if (fromIndex === toIndex) return;
        const movedTab = tabs.value.splice(fromIndex, 1)[0];
        tabs.value.splice(toIndex, 0, movedTab);
    };

    return { tabs, activeTabId, addTab, removeTab, reorderTabs };
});
