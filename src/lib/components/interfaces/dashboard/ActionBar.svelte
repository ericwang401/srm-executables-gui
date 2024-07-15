<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { toast } from 'svelte-sonner'

    import TimepointEngineSelector from './TimepointEngineSelector.svelte'
    import SettingsPanel from '$lib/components/interfaces/settings/SettingsPanel.svelte'
    import * as Form from "$lib/components/ui/form";

    import type { Form as FormType } from '$lib/types/form'

    export let form: FormType


    // const processData = async () => {
    //     try {
    //         const unprocessedFiles = $inputFiles
    //             .filter(file => !file.outputUuid)
    //             .map(file => file.path)
    //
    //         $inputFiles = $inputFiles.map(file => {
    //             if (!file.outputUuid) {
    //                 file.engineType = $engineType
    //             }
    //
    //             return file
    //         })
    //
    //         await invoke('process_data', {
    //             engineType: $engineType,
    //             inputFiles: unprocessedFiles,
    //             toleranceMultiplier: $toleranceMultiplier,
    //             shouldRemoveNaCalculations: $shouldRemoveNACalculations,
    //         })
    //
    //         toast.success('Data processed successfully')
    //     } catch (e) {
    //         toast.error(e as string)
    //     }
    // }
</script>

<div class="flex items-center pb-4 space-x-3">
    <TimepointEngineSelector {form} />
    <div class="grow" />
    <SettingsPanel {form} />
    <Form.Button>Process data</Form.Button>
</div>
