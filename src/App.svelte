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
    let isMouseEvent = false;
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
         *  There is no need to unlisten to these events because
         *  the app will close when the main window is closed.
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
            if (!focused && !isSettingsOpen && !keepWindowShown && !isMouseEvent) {
                appWindow.hide();
            }
        });

        moveButton.addEventListener('mousedown', () => {
            isMouseEvent = true;
            appWindow.startDragging();
        });

        window.addEventListener('resize', () => {
            isMouseEvent = true;
        });

        window.addEventListener('mousedown', () => {
            isMouseEvent = true;
        });

        window.addEventListener('mouseup', () => {
            isMouseEvent = false;
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
            title: 'Quick Note Settings',
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

<main class="overflow-x-hidden h-screen flex flex-col items-center justify-center p-4">
    {#if keepWindowShown}
        <button on:click={() => appWindow.minimize()} class="absolute top-0 right-20 m-2 p-2 rounded">
            <Minus color="#b3b3b3" size="20" />
        </button>
    {/if}
    <button bind:this={moveButton} class="absolute top-0 right-10 m-2 p-2 rounded">
        <Move color="#b3b3b3" size="20" />
    </button>
    <button on:click={openSettings} class="absolute top-0 right-0 m-2 p-2 rounded">
        <Gear color="#b3b3b3" size="20" />
    </button>
    <label class="settings-color select-none absolute top-0 left-0 m-4">
        <input type="checkbox" class="mx-1" bind:checked={keepWindowShown} />
        Keep window on top
    </label>

    <div class="w-full h-full mt-5 flex flex-col justify-start items-start">
        <input bind:value={noteTitle} class="font-semibold text-xl w-full mt-4 px-3 py-2" placeholder="Note title" />
        <hr />
        <textarea
            bind:this={textArea}
            bind:value={noteText}
            class=" font-normal w-full h-full px-3 py-2 mb-12"
            placeholder="Type your note here..."
        />
    </div>

    <div class="select-none absolute bottom-0 px-4 my-3 w-full flex justify-between">
        <button on:click={clearNote} class="note-button"> Clear Note </button>
        <button on:click={createNote} class="note-button">
            Create Note ( {navigator.platform.includes('Mac') ? '⌘' : 'Ctrl'} + S )
        </button>
    </div>
</main>

<style lang="postcss">
    input,
    textarea {
        @apply rounded-md;
        @apply shadow-sm shadow-gray-950;
        background-color: #2a2a2a;
        outline: none;
        resize: none;
        transition: all 0.2s ease-in-out;
    }

    .note-button {
        @apply flex flex-row justify-center items-center;
        @apply shadow-sm shadow-gray-950;
        @apply select-none resize-none rounded-md px-8 h-7 py-1 my-2 text-sm;
        background-color: #2a2a2a;
        color: #b3b3b3;
    }

    .settings-color {
        color: #b3b3b3;
    }

    input::selection,
    textarea::selection {
        background-color: #382d53;
    }

    button {
        transition: all 0.2s ease-in-out;
        color: #dadada;
    }

    input:focus,
    textarea:focus,
    input:hover,
    textarea:hover,
    button:hover {
        background-color: #2e2e2e;
    }

    hr {
        @apply w-full my-3;
        border-top: 1px solid #363636;
    }
</style>
