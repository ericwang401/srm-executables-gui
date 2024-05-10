<script lang="ts">
    import { DocumentChartBar, Icon } from 'svelte-hero-icons'
    import { open } from '@tauri-apps/api/dialog'
    import { validateFileExtension } from '$lib/utils'

    import { toast } from 'svelte-sonner'
    import { createInputFile, type InputFile, inputFiles } from '$lib/stores/input'


    export let compact = false

    const select = async () => {
        const selected = await open({
            multiple: true,
            filters: [{
                name: 'Input Data',
                extensions: ['csv'],
            }],
        })

        if (!Array.isArray(selected)) return

        const files: InputFile[] = selected
            .filter(path => validateFileExtension(path, ['csv']))
            .filter((path) => !$inputFiles.some(file => file.path === path))
            .map(path => createInputFile(path))

        if (files.length !== selected.length) {
            toast.warning('Some files were removed because they weren\'t CSVs or are duplicates')
        }

        $inputFiles = [...files, ...$inputFiles]
    }
</script>

{#if !compact}
    <div
        role="button"
        tabindex="0"
        on:click={select}
        class="p-12 flex flex-col items-center justify-center w-full mt-4 border-2 border-dashed border-slate-300 rounded-md">
        <Icon src={DocumentChartBar} class="w-12 h-12 text-slate-400" />
        <h3 class="font-medium text-sm mt-3">No data</h3>
        <p class="text-muted-foreground text-sm mt-1 font-light">Drag n' drop .csv spreadsheets or click here to select
            input data.</p>
    </div>
{:else}
    <div
        role="button"
        tabindex="0"
        on:click={select}
        class="p-3.5 flex flex-col items-center justify-center w-full mt-4 border-2 border-dashed border-slate-300 rounded-md">
        <p class="text-muted-foreground text-sm font-light">Drag n' drop .csv spreadsheets or click here to select
            input data.</p>
    </div>
{/if}