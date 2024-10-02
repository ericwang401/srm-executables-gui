<script lang="ts">
    import EmptyStateInputSelector from '$lib/components/interfaces/dashboard/InputFileSelector.svelte'
    import InputData from '$lib/components/interfaces/dashboard/InputData.svelte'

    import type { Form as FormType, InputFile } from '$lib/types/form'
    import { listen } from '@tauri-apps/api/event'
    import { onMount } from 'svelte'

    export let form: FormType

    const { form: formData } = form

    const deleteInputFile = (uuid: string) => {
        formData.update(data => {
            return {
                ...data,
                inputFiles: data.inputFiles.filter(file => file.uuid !== uuid),
            }
        })
    }

    interface ProgressSetPayload {
        uuid: string
        iterations: number
        total_iterations: number
    }

    interface ProgressUpdatePayload {
        uuid: string
        iterations: number
    }

    interface ErrorPayload {
        uuid: string
        message: string
    }

    const updateFile = (uuid: string, callback: (file: InputFile) => InputFile) => {
        formData.set(({
            ...$formData,
            inputFiles: $formData.inputFiles.map(f => f.uuid === uuid ? callback(f) : f),
        }))
    }

    onMount(() => {
        setTimeout(() => {
            formData.update(data => ({
                ...data,
                inputFiles: data.inputFiles.map(file => ({
                    ...file,
                    errors: 'lmao'
                }))
            }));
        }, 5000)

        let unlistenProgressSet: () => void
        let unlistenProgressUpdate: () => void
        let unlistenError: () => void

        const main = async () => {
            unlistenProgressSet = await listen('progress-set', (event) => {
                const { uuid, iterations, total_iterations } = event.payload as ProgressSetPayload
                updateFile(uuid, file => ({ ...file, iterations, totalIterations: total_iterations }))
            })

            unlistenProgressUpdate = await listen('progress-update', (event) => {
                const { uuid, iterations } = event.payload as ProgressUpdatePayload
                updateFile(uuid, file => ({ ...file, iterations }))
            })

            unlistenError = await listen('process-error', (event) => {
                console.log('error', { event })
                const { uuid, message } = event.payload as ErrorPayload
                updateFile(uuid, file => ({ ...file, errors: message }))
            })
        }

        main()

        return () => {
            unlistenProgressSet()
            unlistenProgressUpdate()
            unlistenError()
        }
    })
</script>


<EmptyStateInputSelector {form} />

<div class="flex flex-col space-y-1 mt-4">
    {#each $formData.inputFiles as inputFile}
        <InputData on:delete={() => deleteInputFile(inputFile.uuid)} {inputFile} />
    {/each}
</div>