<script lang="ts">
    import { toast } from 'svelte-sonner'
    import * as Card from '$lib/components/ui/card'
    import { invoke } from '@tauri-apps/api/tauri'
    import { Button } from '$lib/components/ui/button'

    let loading = false

    const installDependencies = async () => {
        loading = true
        try {
            await invoke('install_dependencies')

            toast.success('Dependencies installed successfully')

            loading = false
        } catch (e) {
            toast.error((e as Error).message)

            loading = false
            return
        }
    }
</script>

<Card.Root class="col-span-1 md:col-span-2">
    <Card.Header>
        <Card.Title>Install Dependencies</Card.Title>
    </Card.Header>
    <Card.Content class="flex flex-col justify-between space-y-4">
        <p class="text-sm text-muted-foreground">
            Download and load the <a
                class="font-medium underline underline-offset-4"
                href="https://github.com/rgsadygov/SRM_executables"
                target="_blank"
            >
                SRM Executables binaries
            </a>
            from online. This is necessary before this tool can be used.
        </p>

        <Button {loading} on:click={installDependencies}>Install dependencies</Button>
    </Card.Content>
</Card.Root>
