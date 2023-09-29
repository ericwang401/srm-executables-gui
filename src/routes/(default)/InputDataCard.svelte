<script lang="ts">
    import * as Card from '$lib/components/ui/card'
    import { Button } from '$lib/components/ui/button'
    import { Icon, DocumentChartBar } from 'svelte-hero-icons'
    import {  writeBinaryFile, BaseDirectory, writeTextFile, exists, createDir } from '@tauri-apps/api/fs';


    let isSelectingInputData = false
    let inputData: File | null = null

    let isSelectingHeavyWaterInputData = false
    let heavyWaterInputData: File | null = null

    const createDataFolderIfDoesntExist = async () => {
        const folderExists = await exists('data', { dir: BaseDirectory.AppLocalData})
        if (!folderExists) {
            await createDir('data', { dir: BaseDirectory.AppLocalData })
        }
    }

    const selectInputData = async () => {
        isSelectingInputData = true
        try {
            const [fileHandle] = await window.showOpenFilePicker({
                types: [
                    {
                        description: 'Input data',
                        accept: {
                            'text/csv': ['.csv'],
                        },
                    },
                ],
            })
            const file = await fileHandle.getFile()

            inputData = file

            const data = await inputData.arrayBuffer()

            await createDataFolderIfDoesntExist()
            writeBinaryFile('data/Data.csv', data, {
                dir: BaseDirectory.AppLocalData,
            })

        } catch {}
        isSelectingInputData = false
    }

    const selectHeavyWaterInputData = async () => {
        isSelectingHeavyWaterInputData = true
        try {
            const [fileHandle] = await window.showOpenFilePicker()
            const file = await fileHandle.getFile()

            heavyWaterInputData = file

            const data = await heavyWaterInputData.arrayBuffer()

            await createDataFolderIfDoesntExist()
            writeBinaryFile('data/HeavyWater_Data.txt', data, {
                dir: BaseDirectory.AppLocalData,
            })
        } catch {}
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
                class:border-transparent={!inputData}
            >
                <div class="fade-in-right flex items-center space-x-2" class:hidden={!inputData}>
                    <Icon src={DocumentChartBar} class="h-6 w-6 shrink-0" solid />
                    <p class="truncate text-sm">{inputData?.name}</p>
                </div>
            </div>
        </div>
        <div class="flex space-x-2">
            <Button class="shrink-0" on:click={selectHeavyWaterInputData} disabled={isSelectingHeavyWaterInputData}>
                Select heavy water data
            </Button>
            <div
                class="h-10 grow overflow-hidden rounded-md border p-2 transition"
                class:border-transparent={!heavyWaterInputData}
            >
                <div class="fade-in-right flex items-center space-x-2" class:hidden={!heavyWaterInputData}>
                    <Icon src={DocumentChartBar} class="h-6 w-6 shrink-0" solid />
                    <p class="truncate text-sm">{heavyWaterInputData?.name}</p>
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
