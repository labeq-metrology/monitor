import {defineStore} from "pinia";
import {reactive} from "vue";
import {AppState, Folder, Item, LabeqWorkspace} from "@/lib/labeq.ts";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

export const useWorkspaceStore = defineStore('workspaces', () => {
    const workspaces = reactive<Map<string, LabeqWorkspace>>(new Map());

    const createWorkspace = async (name: string) => invoke<string>('create_workspace', {name});
    const deleteWorkspace = async (workspaceId: string) => invoke<void>('delete_workspace', {workspaceId});
    const renameWorkspace = async (workspaceId: string, newName: string) => invoke<void>('rename_workspace', {
        workspaceId,
        newName
    });

    const addDeviceToWorkspace = async (workspaceId: string, connectionRef: string, uuid: string) => invoke<void>('add_device_to_workspace', {
        workspaceId,
        connectionRef,
        uuid
    });
    const removeDeviceFromWorkspace = async (workspaceId: string, deviceId: string) => invoke<void>('remove_device_from_workspace', {
        workspaceId,
        deviceId
    });
    const createDeviceFolder = async (workspaceId: string, parentId: string | undefined, folderName: string) => invoke<void>('create_device_folder', {
        workspaceId,
        parentId,
        folderName
    });
    const renameDeviceFolder = async (workspaceId: string, folderId: string, newName: string) => invoke<void>('rename_device_folder', {
        workspaceId,
        folderId,
        newName
    });
    const deleteDeviceFolder = async (workspaceId: string, folderId: string) => invoke<void>('delete_device_folder', {
        workspaceId,
        folderId
    });
    const moveDeviceToFolder = async (workspaceId: string, deviceId: string, folderId: string | undefined) => invoke<void>('move_device_to_folder', {
        workspaceId,
        deviceId,
        folderId
    });

    const addConnectionToWorkspace = async (workspaceId: string, connectionId: string) => invoke<void>('add_connection_to_workspace', {
        workspaceId,
        connectionId
    });
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
    const removeConnectionFromWorkspace = async (workspaceId: string, connectionId: string) => invoke<void>('remove_connection_from_workspace', {
        workspaceId,
        connectionId
    });
    const createConnectionFolder = async (workspaceId: string, parentId: string | undefined, folderName: string) => invoke<void>('create_connection_folder', {
        workspaceId,
        parentId,
        folderName
    });
    const renameConnectionFolder = async (workspaceId: string, folderId: string, newName: string) => invoke<void>('rename_connection_folder', {
        workspaceId,
        folderId,
        newName
    });
    const deleteConnectionFolder = async (workspaceId: string, folderId: string) => invoke<void>('delete_connection_folder', {
        workspaceId,
        folderId
    });
    const moveConnectionToFolder = async (workspaceId: string, connectionId: string, folderId: string | undefined) => invoke<void>('move_connection_to_folder', {
        workspaceId,
        connectionId,
        folderId
    });

    invoke<AppState>('get_state').then(state => {
        state.workspaces.forEach(workspace => workspaces.set(workspace.id, workspace));
    });

    listen<AppState>('app-state-update', (e) => {
        const ids = new Set(e.payload.workspaces.map(w => w.id));
        for (const key of workspaces.keys()) {
            if (!ids.has(key)) workspaces.delete(key);
        }
        e.payload.workspaces.forEach(workspace => workspaces.set(workspace.id, workspace));
    });

    return {
        workspaces,
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
