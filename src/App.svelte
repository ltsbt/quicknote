<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { dialog } from '@tauri-apps/api';

    let folderPath = '';
    let noteTitle = '';
    let noteText = '';

    function createNote() {
        if (folderPath === '') return;
        if (noteTitle === '') return;
        if (noteText === '') return;

        invoke('create_note', {
            vaultPath: folderPath,
            noteTitle: noteTitle,
            noteContent: noteText,
        });

        noteTitle = '';
        noteText = '';
    }

    async function selectFolder() {
        try {
            const selected = await dialog.open({ directory: true, multiple: false });
            if (Array.isArray(selected)) return;
            if (selected === null) return;
            folderPath = selected;
        } catch (error: any) {
            window.alert(error);
        }
    }
</script>

<main class="overflow-x-hidden flex flex-col items-center justify-center">
    <h1 class="text-4xl m-4">New Note:</h1>
    <button on:click={selectFolder} class="font-bold py-2 px-4 rounded">Choose Note Folder</button>
    <input bind:value={noteTitle} class="w-full my-4 p-2" placeholder="Note title" />
    <textarea bind:value={noteText} class="w-full h-32 p-2" placeholder="Note content" />
    <button on:click={createNote} class="font-bold my-4 py-2 px-4 rounded">Create Note</button>
</main>
