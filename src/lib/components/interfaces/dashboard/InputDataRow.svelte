<script lang="ts">
    import { getFileName } from '$lib/utils'
    import { DocumentChartBar, Icon, XMark } from 'svelte-hero-icons'
    import { inputFiles, type InputFile } from '$lib/stores/input'

    export let inputFile: InputFile
    export let index: number

    let name: string
    $: name = getFileName(inputFile.path, false)

    const remove = () => {
        $inputFiles = $inputFiles.filter((_, i) => i !== index)
    }
</script>

<div class="flex items-center flex-nowrap pl-5 py-3 pr-4 w-full bg-muted first:rounded-t-md last:rounded-b-md">
    <Icon src={DocumentChartBar} solid class="w-6 h-6 mr-2" />
    <span>{name}</span>
    <div class="grow"></div>
    <button on:click={remove} class="h-full p-1 rounded-md hover:bg-card">
        <Icon src={XMark} class="w-5 h-5" />
    </button>
</div>