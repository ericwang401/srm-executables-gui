<script lang="ts">
    import EmptyStateInputSelector from '$lib/components/interfaces/dashboard/InputFileSelector.svelte'
    import InputData from '$lib/components/interfaces/dashboard/InputData.svelte'

    import type { Form as FormType } from '$lib/types/form'

    export let form: FormType

    const { form: formData } = form

    const deleteInputFile = (uuid: string) => {
        $formData.inputFiles = $formData.inputFiles.filter(file => file.uuid !== uuid)
    }
</script>

<EmptyStateInputSelector {form} />

<div class="flex flex-col space-y-1 mt-4">
    {#each $formData.inputFiles as inputFile}
        <InputData on:delete={() => deleteInputFile(inputFile.uuid)} {inputFile} />
    {/each}
</div>