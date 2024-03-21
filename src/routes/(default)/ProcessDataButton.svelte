<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import { invoke } from '@tauri-apps/api/tauri'
    import { toast } from 'svelte-sonner'
    import { inputFile, shouldRemoveNACalculations } from './InputDataCard.svelte'
    import * as Tooltip from '$lib/components/ui/tooltip'

    let loading = false

    const processData = async () => {
        loading = true
        try {
            await invoke('process_data', {
                inputFilePath: $inputFile,
                shouldRemoveNaCalculations: $shouldRemoveNACalculations,
            })

            toast.success('Data processed successfully')

            loading = false
        } catch (e) {
            toast.error(e as string)

            loading = false
            return
        }
    }
</script>

<div class="flex justify-end pb-4">
    {#if $inputFile}
        <Button {loading} on:click={processData}>Process data</Button>
    {:else}
        <Tooltip.Root openDelay={0}>
            <Tooltip.Trigger>
                <Button disabled {loading} on:click={processData}>Process data</Button>
            </Tooltip.Trigger>
            <Tooltip.Content class="max-w-md">
                <p>Specify all the input data first before processing them</p>
            </Tooltip.Content>
        </Tooltip.Root>
    {/if}
</div>
