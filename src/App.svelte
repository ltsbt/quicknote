<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    let text = '';

    function createNote() {
        invoke('create_note', { text });
        text = '';
    }
</script>

<main class="overflow-x-hidden flex flex-col items-center justify-center">
    <h1 class="text-4xl m-8">New Note:</h1>
    <textarea bind:value={text} class="block w-full h-full p-2" placeholder="Note content" />
    <button on:click={createNote} class="font-bold py-2 px-4 rounded">Create Note</button>
</main>
