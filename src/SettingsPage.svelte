<script lang="ts">
    import { dialog } from '@tauri-apps/api';
    import { createDir, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
    import { appConfigDir, sep } from '@tauri-apps/api/path';
    import { appWindow } from '@tauri-apps/api/window';
    import { emit } from '@tauri-apps/api/event';
    import { onMount, onDestroy } from 'svelte';

    import { FolderSearch } from 'lucide-svelte';

    const SETTINGS_DIR = 'settings' + sep;
    const SETTINGS_FILE_NAME = 'settings.json';

    let unlisten: () => void;

    let settings = {
        vaultPath: '',
        useSubfolder: false,
        subfolderPath: '',
    };

    $: vaultName = settings.vaultPath.split(sep).pop();

    onMount(async () => {
        unlisten = await appWindow.onCloseRequested(() => {
            emit('close_settings');
        });

        readSettings();
    });

    onDestroy(() => {
        unlisten();
    });

    /**************************************************************************
     *                  LOAD SETTINGS
     * - vaultPath: string
     * - useSubfolder: boolean
     * - subfolderPath: string
     **************************************************************************/
    async function readSettings() {
        const configDirPath = await appConfigDir();
        const settingsPath = configDirPath + SETTINGS_DIR + SETTINGS_FILE_NAME;

        let settingsFile;

        try {
            settingsFile = await readTextFile(settingsPath);
        } catch {
            // If settings file doesn't exist, create it
            createDir(configDirPath + SETTINGS_DIR, { recursive: true });
            writeTextFile(settingsPath, '{}');
            settingsFile = '{}';
        }

        let json = JSON.parse(settingsFile);
        if (json.vaultPath) settings.vaultPath = json.vaultPath;
        if (json.useSubfolder) settings.useSubfolder = json.useSubfolder;
        if (json.folderPath) settings.subfolderPath = json.folderPath;
    }

    function centerMainWindow() {
        emit('center_window');
    }

    async function selectVault() {
        try {
            const selected = await dialog.open({ directory: true, multiple: false });
            if (Array.isArray(selected)) return;
            if (selected === null) return;
            settings.vaultPath = selected;
        } catch (error: any) {
            window.alert(error);
        }
    }

    async function saveSettings() {
        try {
            const configDirPath = await appConfigDir();
            const settingsPath = configDirPath + SETTINGS_DIR + SETTINGS_FILE_NAME;
            await writeTextFile(settingsPath, JSON.stringify(settings, null, 4));
            emit('update_settings');
        } catch (error: any) {
            window.alert(error);
        }
    }
</script>

<main data-tauri-drag-region class="w-screen h-screen flex flex-col justify-start items-start p-4 overflow-hidden">
    <div class="setting">
        <div>
            <p class="text-md">Vault</p>
            <p class="description">Obsidian vault path to write notes to</p>
        </div>
        <button on:click={selectVault}>
            <FolderSearch size="20" />
            <span class="mx-1"></span>
            {vaultName}
        </button>
    </div>

    <hr />

    <div class="setting">
        <div>
            <p class="title">Use subfolder</p>
            <p class="description">Write notes to a subfolder in the vault</p>
        </div>
        <input type="checkbox" bind:checked={settings.useSubfolder} />
    </div>

    <hr />

    <div class="setting">
        <div>
            <p class="title">Subfolder</p>
            <p class="description">Subfolder path to write notes to</p>
        </div>
        <input type="text" bind:value={settings.subfolderPath} disabled={!settings.useSubfolder} />
    </div>

    <hr />

    <div class="setting">
        <div>
            <p class="title">Center Window</p>
            <p class="description">Reset window position to center of screen</p>
        </div>
        <button on:click={centerMainWindow}>Center Main Window</button>
    </div>
    <hr />
    <div class="flex flex-row justify-end w-full">
        <button on:click={() => readSettings()}>Revert Changes</button>
        <span class="mx-2"></span>
        <button on:click={saveSettings}>Save Settings</button>
    </div>
</main>

<style lang="postcss">
    button {
        @apply flex flex-row justify-center items-center;
        @apply shadow-sm shadow-gray-950;
        @apply select-none resize-none rounded-md h-7 px-3 py-4 my-2 text-sm;
        background-color: #363636;
        color: #dadada;
    }

    button:hover {
        cursor: default;
        background-color: #3f3f3f;
    }
    .setting {
        @apply flex flex-row justify-between items-center w-full;
    }
    input {
        @apply rounded-md px-3 py-2;
        @apply text-sm;
        @apply shadow-sm shadow-gray-950;
        background-color: #2a2a2a;
        border: 1px solid #3f3f3f;
        color: #b3b3b3;
        outline: none;
        resize: none;
        transition: all 0.2s ease-in-out;
    }

    input:disabled {
        color: #666;
    }

    input::selection {
        background-color: #382d53;
    }

    .title {
        color: #dadada;
    }
    .description {
        @apply text-xs;
        color: #b3b3b3;
    }
    hr {
        @apply w-full my-2;
        border-top: 1px solid #363636;
    }
</style>
