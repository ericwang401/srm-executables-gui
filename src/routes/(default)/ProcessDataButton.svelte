<script lang="ts">
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri'
    import { Button } from '$lib/components/ui/button'
    import { toast } from 'svelte-sonner'
    import { appLocalDataDir, join } from '@tauri-apps/api/path'

    let loading = false

    const processData = async () => {
        loading = true
        try {
            await invoke('process_data')

            toast.success('Data processed successfully')

            loading = false
        } catch (e) {
            toast.error((e as Error).message)

            loading = false
            return
        }
    }
</script>

<div class="flex justify-end py-4">
    <Button {loading} on:click={processData}>Process data</Button>
</div>
