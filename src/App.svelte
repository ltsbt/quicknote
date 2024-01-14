<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
    import { listen } from '@tauri-apps/api/event';

    import { onMount } from 'svelte';
    import { Settings as Gear, Move, Minus } from 'lucide-svelte';

    let noteTitle = '';
    let noteText = '';

    let textArea: HTMLTextAreaElement;
    let moveButton: HTMLButtonElement;

    // Variables for whether to hide window when it loses focus
    let isSettingsOpen = false;
    let isDragging = false;
    let keepWindowShown = false;

    let settingsWindow: WebviewWindow;

    // if checkbox is ticked, show taskbar icon and keep window on top
    $: {
        appWindow.setSkipTaskbar(!keepWindowShown);
        appWindow.setAlwaysOnTop(keepWindowShown);
    }

    onMount(async () => {
        /**************************************************************************
         *                  LISTENERS
         **************************************************************************/

        listen('open_main', async () => {
            appWindow.show();
            appWindow.setFocus();
            textArea && textArea.focus();
            if (await appWindow.isMinimized()) appWindow.unminimize();
        });

        listen('new_note', async () => {
            clearNote();
            appWindow.show();
            appWindow.setFocus();
            textArea && textArea.focus();
        });

        listen('open_settings', () => {
            openSettings();
        });

        listen('close_settings', () => {
            isSettingsOpen = false;
        });

        listen('center_window', () => {
            appWindow.center();
        });

        listen('error', ({ payload: errorMessage }) => {
            window.alert(errorMessage);
        });

        appWindow.onFocusChanged(({ payload: focused }) => {
            if (!focused && !isSettingsOpen && !keepWindowShown && !isDragging) {
                appWindow.hide();
            }
        });

        moveButton.addEventListener('mousedown', () => {
            isDragging = true;
            appWindow.startDragging();
        });

        window.addEventListener('mouseup', () => {
            isDragging = false;
        });
    });

    function createNote() {
        if (noteText === '') return;

        // TODO

        noteTitle = '';
        noteText = '';

        appWindow.hide();
    }

    function openSettings() {
        if (isSettingsOpen) return settingsWindow.setFocus();
        isSettingsOpen = true;

        // create settings window
        settingsWindow = new WebviewWindow('settings', {
            title: 'Settings',
            url: '/settingsPage.html',
            center: true,
            width: 600,
            height: 400,
            resizable: true,
        });

        settingsWindow.show();
        settingsWindow.center();
        settingsWindow.setAlwaysOnTop(true);
    }

    function clearNote() {
        noteTitle = '';
        noteText = '';
    }
</script>

<main class="overflow-x-hidden flex flex-col items-center justify-center">
    {#if keepWindowShown}
        <button on:click={() => appWindow.minimize()} class="absolute top-0 right-20 m-2 p-2 rounded">
            <Minus size="20" />
        </button>
    {/if}
    <button bind:this={moveButton} class="absolute top-0 right-10 m-2 p-2 rounded">
        <Move size="20" />
    </button>
    <button on:click={openSettings} class="absolute top-0 right-0 m-2 p-2 rounded">
        <Gear size="20" />
    </button>
    <label class="select-none absolute top-0 left-0 mx-8 mt-4">
        <input type="checkbox" bind:checked={keepWindowShown} />
        Keep window on top
    </label>

    <div class="w-11/12 mt-10">
        <input bind:value={noteTitle} class="font-semibold text-xl w-full my-4 p-2" placeholder="Note title" />
        <textarea
            bind:this={textArea}
            bind:value={noteText}
            class=" font-normal w-full p-2"
            placeholder="Type your note here..."
        />
    </div>

    <div class="select-none absolute bottom-0 my-4 w-11/12 flex justify-between">
        <button on:click={clearNote} class="px-8 py-1 rounded"> Clear Note </button>
        <button on:click={createNote} class=" px-8 py-1 rounded">
            Create Note ( {navigator.platform.includes('Mac') ? '⌘' : 'Ctrl'} + S )
        </button>
    </div>
</main>

<style>
    input,
    textarea {
        border-radius: 0.25rem;
        background-color: #1e1e1e;
        outline: none;
        resize: none;
        transition: all 0.2s ease-in-out;
    }

    textarea {
        height: 75vh;
    }

    input::selection,
    textarea::selection {
        background-color: #382d53;
    }

    button {
        transition: all 0.2s ease-in-out;
    }

    input:focus,
    textarea:focus,
    input:hover,
    textarea:hover,
    button:hover {
        background-color: #2e2e2e;
    }
</style>
