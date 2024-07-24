<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import { invoke } from '@tauri-apps/api/tauri'
    import { toast } from 'svelte-sonner'
    import { shouldRemoveNACalculations, toleranceMultiplier } from './SettingsCard.svelte'

    import { open } from '@tauri-apps/api/dialog'

    let loading = false

    const processData = async () => {
        loading = true
        try {
            const selected = await open({
                filters: [{
                    name: 'Spreadsheet',
                    extensions: ['csv'],
                }],
            })

            if (!selected || Array.isArray(selected)) return

            await invoke('process_data', {
                inputFilePath: selected,
                // @ts-expect-error: I'm doing this because for some reason toleranceMultipler can randomly turn into a string
                toleranceMultiplier: parseFloat($toleranceMultiplier),
                shouldRemoveNaCalculations: $shouldRemoveNACalculations,
            })

            toast.success('Data processed successfully')
        } catch (e) {
            toast.error(e as string)
        } finally {
            loading = false
        }
    }
</script>

<div class="flex justify-end pb-4">
    <Button {loading} on:click={processData}>Process data</Button>
</div>
