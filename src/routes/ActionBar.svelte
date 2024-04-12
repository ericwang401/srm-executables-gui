<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import { invoke } from '@tauri-apps/api/tauri'
    import { toast } from 'svelte-sonner'
    import { shouldRemoveNACalculations, toleranceMultiplier } from './SettingsCard.svelte'

    import { open } from '@tauri-apps/api/dialog'
    import TimepointEngineSelector from './TimepointEngineSelector.svelte'
    import SettingsPanel from '$lib/components/interfaces/settings/SettingsPanel.svelte'

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
                toleranceMultiplier: $toleranceMultiplier,
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

<div class="flex items-center pb-4 space-x-3">
    <TimepointEngineSelector />
    <div class="grow" />
    <SettingsPanel />
    <Button {loading} on:click={processData}>Process data</Button>
</div>
