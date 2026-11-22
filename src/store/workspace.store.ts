import {defineStore} from "pinia";
import {reactive, ref} from "vue";
import {AppState, Folder, Item, LabeqWorkspace, WorkspaceLoadReport} from "@/lib/labeq.ts";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

export const useWorkspaceStore = defineStore('workspaces', () => {
    const workspaces = reactive<Map<string, LabeqWorkspace>>(new Map());
    const dirtyWorkspaceIds = reactive<Set<string>>(new Set());
    const loadReport = ref<WorkspaceLoadReport | null>(null);

    const markWorkspaceDirty = async (workspaceId: string) => {
        await invoke<void>('mark_workspace_dirty', {workspaceId});
        dirtyWorkspaceIds.add(workspaceId);
    };

    const createWorkspace = async (name: string) => {
        const workspaceId = await invoke<string>('create_workspace', {name});
        await markWorkspaceDirty(workspaceId);
        return workspaceId;
    };
    const deleteWorkspace = async (workspaceId: string) => {
        await invoke<void>('delete_workspace', {workspaceId});
        dirtyWorkspaceIds.add(workspaceId);
    };
    const renameWorkspace = async (workspaceId: string, newName: string) => {
        await invoke<void>('rename_workspace', {workspaceId, newName});
        await markWorkspaceDirty(workspaceId);
    };

    const addDeviceToWorkspace = async (workspaceId: string, connectionRef: string, uuid: string) => {
        await invoke<void>('add_device_to_workspace', {workspaceId, connectionRef, uuid});
        await markWorkspaceDirty(workspaceId);
    };
    const removeDeviceFromWorkspace = async (workspaceId: string, deviceId: string) => {
        await invoke<void>('remove_device_from_workspace', {workspaceId, deviceId});
        await markWorkspaceDirty(workspaceId);
    };
    const createDeviceFolder = async (workspaceId: string, parentId: string | undefined, folderName: string) => {
        await invoke<void>('create_device_folder', {workspaceId, parentId, folderName});
        await markWorkspaceDirty(workspaceId);
    };
    const renameDeviceFolder = async (workspaceId: string, folderId: string, newName: string) => {
        await invoke<void>('rename_device_folder', {workspaceId, folderId, newName});
        await markWorkspaceDirty(workspaceId);
    };
    const deleteDeviceFolder = async (workspaceId: string, folderId: string) => {
        await invoke<void>('delete_device_folder', {workspaceId, folderId});
        await markWorkspaceDirty(workspaceId);
    };
    const moveDeviceToFolder = async (workspaceId: string, deviceId: string, folderId: string | undefined) => {
        await invoke<void>('move_device_to_folder', {workspaceId, deviceId, folderId});
        await markWorkspaceDirty(workspaceId);
    };

    const addConnectionToWorkspace = async (workspaceId: string, connectionId: string) => {
        await invoke<void>('add_connection_to_workspace', {workspaceId, connectionId});
        await markWorkspaceDirty(workspaceId);
    };
    const workspaceHasConnection = (workspaceId: string, connectionId: string) => {
        const workspace = workspaces.get(workspaceId);
        if (!workspace) return false;

        const containsConnection = (items: Item<string>[]): boolean => items.some(item =>
            item.type === 'item'
                ? item.value === connectionId
                : containsConnection(item.value.items)
        );

        return containsConnection((workspace.connections as Folder<string>).items);
    };
    const removeConnectionFromWorkspace = async (workspaceId: string, connectionId: string) => {
        await invoke<void>('remove_connection_from_workspace', {workspaceId, connectionId});
        await markWorkspaceDirty(workspaceId);
    };
    const createConnectionFolder = async (workspaceId: string, parentId: string | undefined, folderName: string) => {
        await invoke<void>('create_connection_folder', {workspaceId, parentId, folderName});
        await markWorkspaceDirty(workspaceId);
    };
    const renameConnectionFolder = async (workspaceId: string, folderId: string, newName: string) => {
        await invoke<void>('rename_connection_folder', {workspaceId, folderId, newName});
        await markWorkspaceDirty(workspaceId);
    };
    const deleteConnectionFolder = async (workspaceId: string, folderId: string) => {
        await invoke<void>('delete_connection_folder', {workspaceId, folderId});
        await markWorkspaceDirty(workspaceId);
    };
    const moveConnectionToFolder = async (workspaceId: string, connectionId: string, folderId: string | undefined) => {
        await invoke<void>('move_connection_to_folder', {workspaceId, connectionId, folderId});
        await markWorkspaceDirty(workspaceId);
    };

    invoke<AppState>('get_state').then(state => {
        state.workspaces.forEach(workspace => workspaces.set(workspace.id, workspace));
        dirtyWorkspaceIds.clear();
        state.dirtyWorkspaces?.forEach(workspaceId => dirtyWorkspaceIds.add(workspaceId));
    });

    listen<AppState>('app-state-update', (e) => {
        const ids = new Set(e.payload.workspaces.map(w => w.id));
        for (const key of workspaces.keys()) {
            if (!ids.has(key)) workspaces.delete(key);
        }
        e.payload.workspaces.forEach(workspace => workspaces.set(workspace.id, workspace));
        dirtyWorkspaceIds.clear();
        e.payload.dirtyWorkspaces?.forEach(workspaceId => dirtyWorkspaceIds.add(workspaceId));
    });
    listen<WorkspaceLoadReport>('workspace-load-report', (event) => {
        loadReport.value = event.payload;
    });

    const saveWorkspace = async (workspaceId: string) => {
        await invoke<string>('save_workspace', {workspaceId});
        dirtyWorkspaceIds.delete(workspaceId);
    };
    const saveWorkspaceAs = async (workspaceId: string) => {
        await invoke<string>('save_workspace_as', {workspaceId});
        dirtyWorkspaceIds.delete(workspaceId);
    };
    const openWorkspace = async () => {
        const report = await invoke<WorkspaceLoadReport>('open_workspace');
        loadReport.value = report;
        return report;
    };
    const clearLoadReport = () => {
        loadReport.value = null;
    };

    const countDevicesForConnection = (workspaceId: string, connectionId: string) => {
        const workspace = workspaces.get(workspaceId);
        if (!workspace) return 0;
        const count = (items: Item<unknown>[]): number => items.reduce((total, item) => {
            if (item.type === 'folder') return total + count(item.value.items);
            const tuple = item.value as [string, string];
            return total + (tuple[0] === connectionId ? 1 : 0);
        }, 0);
        return count(workspace.devices.items as Item<unknown>[]);
    };

    return {
        workspaces,
        dirtyWorkspaceIds,
        saveWorkspace,
        saveWorkspaceAs,
        openWorkspace,
        clearLoadReport,
        countDevicesForConnection,
        loadReport,
        createWorkspace,
        deleteWorkspace,
        renameWorkspace,

        addDeviceToWorkspace,
        removeDeviceFromWorkspace,
        createDeviceFolder,
        renameDeviceFolder,
        deleteDeviceFolder,
        moveDeviceToFolder,

        addConnectionToWorkspace,
        workspaceHasConnection,
        removeConnectionFromWorkspace,
        createConnectionFolder,
        renameConnectionFolder,
        deleteConnectionFolder,
        moveConnectionToFolder,
    };
});
