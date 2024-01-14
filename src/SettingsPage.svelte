<script lang="ts">
    import { dialog } from '@tauri-apps/api';
    import { createDir, BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
    import { appWindow } from '@tauri-apps/api/window';
    import { emit } from '@tauri-apps/api/event';
    import { onMount, onDestroy } from 'svelte';

    let unlisten: () => void;

    let vaultPath = '';
    let folderPath = '';

    onMount(async () => {
        unlisten = await appWindow.onCloseRequested(() => {
            emit('close_settings');
        });

        /**************************************************************************
         *                  LOAD SETTINGS
         **************************************************************************/

        const settingsJson = await readTextFile('settings/settings.json', {
            dir: BaseDirectory.AppLocalData,
        });

        // TODO
    });

    onDestroy(() => {
        unlisten();
    });

    function centerMainWindow() {
        emit('center_window');
    }

    async function selectFolder() {
        try {
            const selected = await dialog.open({ directory: true, multiple: false });
            if (Array.isArray(selected)) return;
            if (selected === null) return;
            const folderPath = selected;
            // TODO
        } catch (error: any) {
            window.alert(error);
        }
    }

    async function saveSettings() {
        // TODO
        emit('update_settings');
    }
</script>

<main data-tauri-drag-region class="w-screen h-screen flex flex-col justify-center items-center">
    <button on:click={centerMainWindow} class="font-bold py-2 px-4 rounded">Center Main Window</button>
</main>
