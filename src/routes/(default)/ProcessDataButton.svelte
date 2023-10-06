<script lang="ts" context="module">
    import { writable } from 'svelte/store'

    export const loading = writable(false)
</script>

<script lang="ts">
    import { Button } from '$lib/components/ui/button'
    import { invoke } from '@tauri-apps/api/tauri'
    import { toast } from 'svelte-sonner'

    const processData = async () => {
        $loading = true
        try {
            await invoke('process_data')

            toast.success('Data processed successfully')

            $loading = false
        } catch (e) {
            toast.error(e as string)

            $loading = false
            return
        }
    }
</script>

<div class="flex justify-end pb-4">
    <Button loading={$loading} on:click={processData}>Process data</Button>
</div>
