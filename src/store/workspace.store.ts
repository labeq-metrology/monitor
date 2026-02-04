import {defineStore} from "pinia";
import {reactive} from "vue";
import {AppState, LabeqConnection, LabeqDevice, LabeqWorkspace} from "@/lib/labeq.ts";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

export const useWorkspaceSore = defineStore('workspaces', () => {
    const workspaces = reactive<Map<string, LabeqWorkspace>>(new Map());

    const createWorkspace = async (name: string) => invoke<string>('create_workspace', {name});
    const deleteWorkspace = async (workspaceId: string) => invoke<void>('delete_workspace', {workspaceId});
    const renameWorkspace = async (workspaceId: string, newName: string) => invoke<void>('rename_workspace', {
        workspaceId,
        newName
    });
    const addDeviceToWorkspace = async (workspaceId: string, device: LabeqDevice) => invoke<void>('add_device_to_workspace', {
        workspaceId,
        device
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

    const addConnectionToWorkspace = async (workspaceId: string, connection: LabeqConnection) => invoke<void>('add_connection_to_workspace', {
        workspaceId,
        connection
    });
    const removeConnectionFromWorkspace = async (workspaceId: string, connectionId: string) => invoke<void>('remove_connection_from_workspace', {
        workspaceId,
        connectionId
    });

    invoke<AppState>('get_state').then(state => {
        state.workspaces.forEach(workspace => workspaces.set(workspace.id, workspace));
    });

    listen<AppState>('app-state-update', (e) => {
        e.payload.workspaces.forEach(workspace => workspaces.set(workspace.id, workspace));
    }).then(_ => {
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
        removeConnectionFromWorkspace
    };
});
