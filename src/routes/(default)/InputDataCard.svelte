<script lang="ts" context="module">
    export const inputFile = writable<string | null>(null)
    export const shouldRemoveNACalculations = writable<boolean>(false)

</script>


<script lang="ts">
    import * as Card from '$lib/components/ui/card'
    import { Button } from '$lib/components/ui/button'
    import { Icon, DocumentChartBar, InformationCircle } from 'svelte-hero-icons'
    import { Switch } from '$lib/components/ui/switch'
    import { Label } from '$lib/components/ui/label'
    import * as Tooltip from '$lib/components/ui/tooltip'
    import { writable } from 'svelte/store'
    import { open } from '@tauri-apps/api/dialog'

    let isSelectingInputFile = false
    $: inputFileName = $inputFile ? $inputFile.split('/').pop() : null

    const selectInputFile = async () => {
        isSelectingInputFile = true
        try {
            const selected = await open({
                filters: [{
                    name: 'Spreadsheet',
                    extensions: ['csv'],
                }],
            })

            if (selected && !Array.isArray(selected)) {
                $inputFile = selected
            }

            console.log({selected})
        } catch {
            /* empty */
        }

        isSelectingInputFile = false
    }
</script>

<Card.Root class="col-span-1 md:col-span-2">
    <Card.Header>
        <Card.Title>Input Data</Card.Title>
    </Card.Header>
    <Card.Content class="flex flex-col space-y-4">
        <div class="flex space-x-2">
            <Button class="shrink-0" on:click={selectInputFile} disabled={isSelectingInputFile}>
                Select input data
            </Button>
            <div
                class="h-10 grow overflow-hidden rounded-md border p-2 transition"
                class:border-transparent={!$inputFile}
            >
                <div
                    class="fade-in-right flex items-center space-x-2"
                    class:hidden={!$inputFile}
                >
                    <Icon src={DocumentChartBar} class="h-6 w-6 shrink-0" solid />
                    <p class="truncate text-sm">{inputFileName}</p>
                </div>
            </div>
        </div>

        <div class="flex items-center space-x-2">
            <Switch id="should-remove-na-calculations" bind:checked={$shouldRemoveNACalculations} />
            <Label for="should-remove-na-calculations">Remove N/A Calculations</Label>
            <Tooltip.Root openDelay={0}>
                <Tooltip.Trigger>
                    <Icon src={InformationCircle} mini class='w-4 h-4' />
                </Tooltip.Trigger>
                <Tooltip.Content class='max-w-md'>
                    <p>Removes "-nan(ind)" values from the output by omitting samples that contain no detected peaks for
                        a specific peptide. This results in a lower sample count for affected peptides.</p>
                </Tooltip.Content>
            </Tooltip.Root>
        </div>
    </Card.Content>
</Card.Root>

<style>
    :global(.fade-in-right) {
        animation: fadeInRight 0.25s cubic-bezier(0, 0, 0.2, 1);
    }

    @keyframes fadeInRight {
        0% {
            opacity: 0;
            transform: translateX(-20px);
        }
        100% {
            opacity: 1;
            transform: translateX(0);
        }
    }
</style>
