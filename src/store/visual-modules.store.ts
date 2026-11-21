import {defineStore} from 'pinia';
import {computed, markRaw, ref} from 'vue';
import type {LabeqPluginManifest, PanelPluginManifest} from '@/lib/plugin.ts';

interface VisualModelManifest {
    name: string;
    type: string;
    version: string;
    description: string;
    author?: string;
    icon?: string;
    category?: string;
    configSchema: any[];
    dataSchema?: any;
    metadata?: any;
    panel: PanelPluginManifest;
}

interface LoadedModel {
    id: string;
    manifest: VisualModelManifest;
    component: any;
    basePath: string;
}

export const useVisualModelsStore = defineStore('visualModels', () => {
    const models = ref(new Map<string, LoadedModel>());

    const allModels = computed(() => Array.from(models.value.values()));

    const modelCount = computed(() => models.value.size);

    const isInitialized = computed(() => models.value.size > 0);

    const categories = computed(() => {
        const cats = new Set<string>();
        models.value.forEach(model => {
            if (model.manifest.category) {
                cats.add(model.manifest.category);
            }
        });
        return Array.from(cats);
    });

    function loadEmbeddedPanel(panel: PanelPluginManifest, plugin: LabeqPluginManifest): LoadedModel | null {
        if (!panel.bundle) return null;
        try {
            const component = new Function('Vue', 'Pinia', panel.bundle)(
                (window as any).Vue,
                (window as any).Pinia
            );
            if (!component) throw new Error(`Panel ${panel.id} returned no component`);

            const model = {
                id: panel.id,
                manifest: {
                    name: panel.id,
                    type: 'panel',
                    version: plugin.version,
                    description: plugin.description ?? '',
                    configSchema: [],
                    panel,
                },
                component: markRaw(component),
                basePath: `plugin:${plugin.id}`,
            };
            models.value.set(panel.id, model);
            return model;
        } catch (error) {
            console.error(`Failed to load panel ${panel.id} from plugin ${plugin.id}:`, error);
            return null;
        }
    }

    function loadPluginPanels(plugins: LabeqPluginManifest[]) {
        plugins.forEach(plugin => {
            (plugin.panels ?? []).forEach(panel => loadEmbeddedPanel(panel, plugin));
        });
    }

    function getModel(modelId: string): LoadedModel | undefined {
        return models.value.get(modelId);
    }

    function isActionPanel(panelId: string): boolean {
        return models.value.get(panelId)?.manifest.panel.execute === true;
    }

    function getModelsByCategory(category: string): LoadedModel[] {
        return allModels.value.filter(m => m.manifest.category === category);
    }

    function clearModels() {
        models.value.clear();
        console.log('🗑️  Cleared all models');
    }

    // Return store API
    return {
        // State
        models,
        // Getters
        allModels,
        modelCount,
        isInitialized,
        categories,

        // Actions
        getModel,
        loadPluginPanels,
        isActionPanel,
        getModelsByCategory,
        clearModels
    };
});
