<script lang="ts">
    import { DocumentChartBar, Icon } from 'svelte-hero-icons'
    import { open } from '@tauri-apps/api/dialog'
    import { cn, processInputFiles } from '$lib/utils'

    import { toast } from 'svelte-sonner'
    import type { Form as FormType } from '$lib/types/form'

    export let form: FormType

    const { form: formData } = form

    $: inputFilesLength = $formData.inputFiles.length

    const select = async () => {
        const selected = await open({
            multiple: true,
            filters: [{
                name: 'Input Data',
                extensions: ['csv'],
            }],
        })

        if (!Array.isArray(selected)) return

        const files = processInputFiles(selected)

        if (files.length !== selected.length) {
            toast.warning('Some non-CSV files were removed')
        }

        $formData.inputFiles = [...files, ...$formData.inputFiles]
    }
</script>

<div
    role="button"
    tabindex="0"
    on:click={select}
    on:keypress={select}
    class={cn(
        "flex flex-col items-center justify-center w-full mt-4 border-2 border-dashed border-slate-300 rounded-md",
        inputFilesLength > 0 ? 'p-3.5' : 'p-12'
        )}>
    {#if inputFilesLength === 0}
        <Icon src={DocumentChartBar} class="w-12 h-12 text-slate-400" />
        <h3 class="font-medium text-sm mt-3">No data</h3>
    {/if}
    <p class={cn("text-muted-foreground text-sm font-light", inputFilesLength === 0 && 'mt-1')}>Drag n' drop .csv
        spreadsheets or click here to select
        input data.</p>
</div>