<script lang="ts">
    import * as Card from '$lib/components/ui/card'
    import { Button } from '$lib/components/ui/button'
    import { Icon, DocumentChartBar } from 'svelte-hero-icons'
    import { invoke } from '@tauri-apps/api/tauri'
    import { toast } from 'svelte-sonner'

    let isSelectingInputData = false
    let inputDataName: string | null = null

    let isSelectingHeavyWaterInputData = false
    let heavyWaterInputDataName: string | null = null

    const selectInputData = async () => {
        isSelectingInputData = true
        try {
            const filename: string = await invoke('select_data', {
                dataInputType: 'inputData',
            })

            inputDataName = filename ? filename : inputDataName
        } catch (e) {
            toast.error(e as string)
        }
        isSelectingInputData = false
    }

    const selectHeavyWaterInputData = async () => {
        isSelectingHeavyWaterInputData = true
        try {
            const filename: string = await invoke('select_data', {
                dataInputType: 'heavyWaterInputData',
            })

            heavyWaterInputDataName = filename ? filename : heavyWaterInputDataName
        } catch (e) {
            toast.error(e as string)
        }
        isSelectingHeavyWaterInputData = false
    }
</script>

<Card.Root class="col-span-1 md:col-span-2">
    <Card.Header>
        <Card.Title>Input Data</Card.Title>
    </Card.Header>
    <Card.Content class="flex flex-col space-y-4">
        <div class="flex space-x-2">
            <Button class="shrink-0" on:click={selectInputData} disabled={isSelectingInputData}>
                Select input data
            </Button>
            <div
                class="h-10 grow overflow-hidden rounded-md border p-2 transition"
                class:border-transparent={!inputDataName}
            >
                <div class="fade-in-right flex items-center space-x-2" class:hidden={!inputDataName}>
                    <Icon src={DocumentChartBar} class="h-6 w-6 shrink-0" solid />
                    <p class="truncate text-sm">{inputDataName}</p>
                </div>
            </div>
        </div>
        <div class="flex space-x-2">
            <Button
                class="shrink-0"
                on:click={selectHeavyWaterInputData}
                disabled={isSelectingHeavyWaterInputData}
            >
                Select heavy water data
            </Button>
            <div
                class="h-10 grow overflow-hidden rounded-md border p-2 transition"
                class:border-transparent={!heavyWaterInputDataName}
            >
                <div
                    class="fade-in-right flex items-center space-x-2"
                    class:hidden={!heavyWaterInputDataName}
                >
                    <Icon src={DocumentChartBar} class="h-6 w-6 shrink-0" solid />
                    <p class="truncate text-sm">{heavyWaterInputDataName}</p>
                </div>
            </div>
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
