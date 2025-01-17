<script lang='ts'>
    import { DocumentChartBar, Icon } from 'svelte-hero-icons'
    import { appWindow } from '@tauri-apps/api/window'
    import { onMount } from 'svelte'
    import type { UnlistenFn } from '@tauri-apps/api/event'
    import { processInputFiles } from '$lib/utils'
    import { toast } from 'svelte-sonner'
    import type { Form as FormType } from '$lib/types/form'

    export let form: FormType

    const { form: formData } = form

    let dragging = false
    let animationKey = Math.random()

    onMount(() => {
        let unlisten: UnlistenFn

        const main = async () => {
            unlisten = await appWindow.onFileDropEvent((event) => {
                if (event.payload.type === 'hover') {
                    animationKey = Math.random()
                    dragging = true
                } else if (event.payload.type === 'drop') {
                    dragging = false
                    const files = processInputFiles(event.payload.paths)

                    if (files.length !== event.payload.paths.length) {
                        toast.warning('Some non-CSV files were removed')
                    }

                    $formData.inputFiles = [...files, ...$formData.inputFiles]
                } else {
                    dragging = false
                }
            })
        }

        main()

        return () => unlisten()
    })

</script>

<div
    class="fixed flex flex-col items-center justify-center z-[999999999999] w-full h-full bg-slate-100 top-0 left-0 pointer-events-none transition"
    class:opacity-0={!dragging}
    class:opacity-75={dragging}
>
    <div class="absolute inset-10 border-2 border-dashed border-slate-300" />
    <div class="relative w-20 h-20">
        {#key animationKey}
            <Icon src={DocumentChartBar}
                  class="absolute w-20 h-20 text-slate-600 fill-white dropzone-animated-icon z-[3]"
                  style="--offset: 1rem" />
            <Icon src={DocumentChartBar}
                  class="absolute w-20 h-20 text-slate-600 fill-white dropzone-animated-icon z-[2]" />
            <Icon src={DocumentChartBar}
                  class="absolute w-20 h-20 text-slate-600 fill-white dropzone-animated-icon z-[1]"
                  style="--offset: -1rem" />
        {/key}
    </div>
    <h3 class="mt-4 font-medium">Drop .csv spreadsheets here</h3>
</div>

<style>
    :global(.dropzone-animated-icon) {
        animation: cubic-bezier(0.175, 0.885, 0.32, 1.275) 500ms forwards spread;
    }

    @keyframes spread {
        0% {
            transform: translateX(0) translateY(0);
        }
        100% {
            transform: translateX(calc(var(--offset, 0) * -1)) translateY(var(--offset, 0));
        }
    }
</style>