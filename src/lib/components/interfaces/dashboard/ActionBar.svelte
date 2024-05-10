<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import { invoke } from '@tauri-apps/api/tauri'
    import { toast } from 'svelte-sonner'
    import { engineType, shouldRemoveNACalculations, toleranceMultiplier } from '$lib/stores/settings'

    import TimepointEngineSelector from './TimepointEngineSelector.svelte'
    import SettingsPanel from '$lib/components/interfaces/settings/SettingsPanel.svelte'
    import { inputFiles } from '$lib/stores/input'


    const processData = async () => {
        try {
            const unprocessedFiles = $inputFiles
                .filter(file => !file.outputUuid)
                .map(file => file.path)

            $inputFiles = $inputFiles.map(file => {
                if (!file.outputUuid) {
                    file.engineType = $engineType
                }

                return file
            })

            await invoke('process_data', {
                engineType: $engineType,
                inputFiles: unprocessedFiles,
                toleranceMultiplier: $toleranceMultiplier,
                shouldRemoveNaCalculations: $shouldRemoveNACalculations,
            })

            toast.success('Data processed successfully')
        } catch (e) {
            toast.error(e as string)
        }
    }
</script>

<div class="flex items-center pb-4 space-x-3">
    <TimepointEngineSelector />
    <div class="grow" />
    <SettingsPanel />
    <Button on:click={processData}>Process data</Button>
</div>
