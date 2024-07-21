<script lang="ts">
    import { DocumentChartBar, Icon, XMark } from 'svelte-hero-icons'
    import type { InputFile } from '$lib/types/form'
    import { createEventDispatcher } from 'svelte'

    export let inputFile: InputFile

    const dispatch = createEventDispatcher<{
        delete: void
    }>()

</script>

<div class="flex items-center flex-nowrap pl-5 py-3 pr-4 w-full bg-muted first:rounded-t-md last:rounded-b-md">
    <Icon src={DocumentChartBar} solid class="w-6 h-6 mr-2" />
    <span class="truncate">{inputFile.path.name}</span>
    <div class="grow"></div>
    <button on:click={() => dispatch('delete')} class="h-full p-1 rounded-md hover:bg-card">
        <Icon src={XMark} class="w-5 h-5" />
    </button>
</div>