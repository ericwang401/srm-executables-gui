<script lang="ts">
    import { DocumentChartBar, ExclamationCircle, Icon, XMark } from 'svelte-hero-icons'
    import type { InputFile } from '$lib/types/form'
    import { createEventDispatcher } from 'svelte'

    export let inputFile: InputFile

    const dispatch = createEventDispatcher<{
        delete: void
    }>()

</script>

<div class="flex items-center flex-nowrap pl-5 py-3 pr-4 w-full bg-muted first:rounded-t-md last:rounded-b-md">
    {#if inputFile.errors}
        <Icon src={ExclamationCircle} solid class="w-6 h-6 mr-2 text-destructive" />
    {:else}
        <Icon src={DocumentChartBar} solid class="w-6 h-6 mr-2" />
    {/if}
    <div>
        <p class="truncate">{inputFile.path.name}</p>
        {#if inputFile.errors}
            <p class="truncate text-xs text-muted-foreground">{inputFile.errors}</p>
        {/if}
    </div>
    <div class="grow"></div>
    <button on:click={() => dispatch('delete')} class="h-full p-1 rounded-md hover:bg-card">
        <Icon src={XMark} class="w-5 h-5" />
    </button>
</div>