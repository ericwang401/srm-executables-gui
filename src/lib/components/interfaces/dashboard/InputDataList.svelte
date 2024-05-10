<script lang="ts">
    import EmptyStateInputSelector from '$lib/components/interfaces/dashboard/EmptyStateInputSelector.svelte'
    import InputDataRow from '$lib/components/interfaces/dashboard/InputDataRow.svelte'
    import { inputFiles } from '$lib/stores/input'
</script>

<EmptyStateInputSelector compact={$inputFiles.length > 0} />

<div class="flex flex-col space-y-1 mt-4">
    {#each $inputFiles as inputFile, index}
        <InputDataRow {inputFile} {index} />
    {/each}
</div>