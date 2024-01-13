<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { Settings } from 'lucide-svelte';
    import { appWindow } from '@tauri-apps/api/window';

    let noteTitle = '';
    let noteText = '';

    function createNote() {
        if (noteTitle === '') return;
        if (noteText === '') return;

        // invoke('create_note', {
        //     noteTitle: noteTitle,
        //     noteContent: noteText,
        // });

        noteTitle = '';
        noteText = '';

        appWindow.hide();
    }

    function openSettings() {}
</script>

<main class="overflow-x-hidden flex flex-col items-center justify-center">
    <button on:click={openSettings} class="absolute top-0 right-0 m-4 rounded">
        <Settings size="20" />
    </button>

    <div class="w-11/12 mt-10">
        <input bind:value={noteTitle} class="font-semibold text-xl w-full my-4 p-2" placeholder="Note title" />
        <textarea bind:value={noteText} class="font-normal w-full p-2" placeholder="Type your note here..." />
    </div>

    <button on:click={createNote} class="absolute bottom-0 my-4 px-8 py-2 rounded">Create Note (Ctrl/Cmd + S)</button>
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
        height: 70vh;
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
