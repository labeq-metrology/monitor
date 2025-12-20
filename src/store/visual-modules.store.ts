import {defineStore} from 'pinia';
import {computed, markRaw, ref} from 'vue';
import {readDir, readTextFile} from '@tauri-apps/plugin-fs';
import {appDataDir, join} from '@tauri-apps/api/path';

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
}

interface LoadedModel {
    id: string;
    manifest: VisualModelManifest;
    component: any;
    basePath: string;
}

export const useVisualModelsStore = defineStore('visualModels', () => {
    const models = ref(new Map<string, LoadedModel>());
    const modelsBasePath = ref('');
    const loadedStyleIds = ref(new Set<string>());

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

    async function initBasePath() {
        try {
            const appData = await appDataDir();
            modelsBasePath.value = await join(appData, 'visual-models');
            console.log(`Models path: ${modelsBasePath.value}`);
        } catch (err) {
            console.error('Failed to init base path:', err);
            modelsBasePath.value = 'visual-models';
        }
    }

    async function loadModel(modelId: string, modelPath: string): Promise<LoadedModel | null> {
        try {
            const manifestPath = await join(modelPath, 'manifest.json');
            const manifestContent = await readTextFile(manifestPath);
            const manifest: VisualModelManifest = JSON.parse(manifestContent);

            const bundlePath = await join(modelPath, 'bundle.js');
            const bundleContent = await readTextFile(bundlePath);

            const cssPath = await join(modelPath, 'bundle.css');
            let cssContent = '';
            try {
                cssContent = await readTextFile(cssPath);
            } catch {
                console.warn(`No CSS file for ${modelId}`);
            }

            if (cssContent) {
                const styleId = `visual-model-style-${modelId}`;

                const oldStyle = document.getElementById(styleId);
                if (oldStyle) {
                    oldStyle.remove();
                    loadedStyleIds.value.delete(styleId);
                }

                const styleEl = document.createElement('style');
                styleEl.id = styleId;
                styleEl.textContent = cssContent;
                document.head.appendChild(styleEl);
                loadedStyleIds.value.add(styleId);

                console.log(`Injected CSS for ${modelId}`);
            }

            if (typeof window !== 'undefined') {
                (window as any).Vue = await import('vue');
                (window as any).Pinia = await import('pinia');
            }

            const componentFactory = new Function('Vue', 'Pinia', `
                const exports = {};
                ${bundleContent}
            `);

            const component = componentFactory(
                (window as any).Vue,
                (window as any).Pinia
            );

            if (!component) {
                throw new Error(`Visual model ${modelId} does not export a component`);
            }

            const model: LoadedModel = {
                id: modelId,
                manifest,
                component: markRaw(component),
                basePath: modelPath
            };

            models.value.set(modelId, model);

            console.log(`Loaded: ${manifest.name} v${manifest.version}`);

            return model;
        } catch (err: any) {
            console.error('Load error:', err);
            return null;
        }
    }

    async function autoDiscoverModels() {
        try {
            if (!modelsBasePath.value) {
                await initBasePath();
            }

            let entries;
            try {
                entries = await readDir(modelsBasePath.value);
            } catch (err) {
                console.warn(`Models directory not found: ${modelsBasePath.value}`);
                return;
            }

            const modelDirs = entries.filter(entry => entry.isDirectory);

            if (modelDirs.length === 0) {
                console.warn('No visual models found');
                return;
            }

            const loadPromises = modelDirs.map(async (entry) => {
                try {
                    const modelPath = await join(modelsBasePath.value, entry.name!);
                    return await loadModel(entry.name!, modelPath);
                } catch (err) {
                    console.warn(`Skipping ${entry.name}:`, err);
                    return null;
                }
            });

            const results = await Promise.all(loadPromises);
            const successCount = results.filter(r => r !== null).length;

            console.log(`Successfully loaded ${successCount}/${modelDirs.length} model(s)`);
        } catch (err: any) {
            console.error('Auto-discovery error:', err);
        }
    }

    function getModel(modelId: string): LoadedModel | undefined {
        return models.value.get(modelId);
    }

    function getModelsByCategory(category: string): LoadedModel[] {
        return allModels.value.filter(m => m.manifest.category === category);
    }

    function unloadModel(modelId: string): boolean {
        const model = models.value.get(modelId);
        if (!model) return false;

        // Remove CSS
        const styleId = `visual-model-style-${modelId}`;
        const styleEl = document.getElementById(styleId);
        if (styleEl) {
            styleEl.remove();
            loadedStyleIds.value.delete(styleId);
            console.log(`🗑️  Removed CSS for ${modelId}`);
        }

        // Remove model
        const deleted = models.value.delete(modelId);
        if (deleted) {
            console.log(`🗑️  Unloaded model: ${modelId}`);
        }

        return deleted;
    }

    async function reloadModel(modelId: string): Promise<LoadedModel | null> {
        const existing = models.value.get(modelId);
        if (!existing) {
            return null;
        }

        unloadModel(modelId);
        return await loadModel(modelId, existing.basePath);
    }

    function clearModels() {
        // Remove all CSS
        loadedStyleIds.value.forEach(styleId => {
            const styleEl = document.getElementById(styleId);
            if (styleEl) {
                styleEl.remove();
            }
        });
        loadedStyleIds.value.clear();

        // Clear models
        models.value.clear();
        console.log('🗑️  Cleared all models');
    }

    // Return store API
    return {
        // State
        models,
        modelsBasePath,

        // Getters
        allModels,
        modelCount,
        isInitialized,
        categories,

        // Actions
        initBasePath,
        loadModel,
        autoDiscoverModels,
        getModel,
        getModelsByCategory,
        unloadModel,
        reloadModel,
        clearModels
    };
});
