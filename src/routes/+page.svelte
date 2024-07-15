<script lang='ts'>
    import ActionBar from '$lib/components/interfaces/dashboard/ActionBar.svelte'
    import Dropzone from '$lib/components/interfaces/dashboard/DropzoneOverlay.svelte'
    import InputDataList from '$lib/components/interfaces/dashboard/InputDataList.svelte'
    import { defaults, superForm } from 'sveltekit-superforms'
    import { zod } from 'sveltekit-superforms/adapters'
    import { schema } from '$lib/types/form'
    import { invoke } from '@tauri-apps/api/tauri'


    const form = superForm(defaults({
        engineType: 'single',
        shouldRemoveNACalculations: true,
        toleranceMultiplier: 2.0,
        inputFiles: [],
    }, zod(schema)), {
        resetForm: false,
        SPA: true,
        validators: zod(schema),
        onUpdate: async ({ form }) => {
            if (!form.valid) return

            const { inputFiles, engineType, toleranceMultiplier, shouldRemoveNACalculations } = form.data
            const unprocessedFiles = inputFiles
                .filter(file => !file.isProcessed)
                .map(file => file.path.dir)

            unprocessedFiles.forEach(file => {
                const f = form.data.inputFiles.find(f => f.path.dir === file)
                if (f) {
                    f.engineType = engineType
                }
            })

            try {
                await invoke('process_data', {
                    engineType,
                    inputFiles: unprocessedFiles,
                    toleranceMultiplier,
                    shouldRemoveNaCalculations: shouldRemoveNACalculations,
                })
            } catch (e) {
                console.error(e)
            }
        },
    })

    const { enhance } = form
</script>

<form method="POST" use:enhance class="flex flex-col space-y-8">
    <Dropzone {form} />

    <ActionBar {form} />

    <InputDataList {form} />
</form>