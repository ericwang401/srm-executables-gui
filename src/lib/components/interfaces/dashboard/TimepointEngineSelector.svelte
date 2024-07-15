<script lang="ts">
    import * as Tabs from '$lib/components/ui/tabs'

    import type { Form as FormType } from '$lib/types/form'

    export let form: FormType

    const { form: formData } = form

</script>

<Tabs.Root bind:value={$formData.engineType}>
    <Tabs.List>
        <Tabs.Trigger value="single">Single-Timepoint</Tabs.Trigger>
        <Tabs.Trigger value="multi">Multi-Timepoint</Tabs.Trigger>
    </Tabs.List>
</Tabs.Root>