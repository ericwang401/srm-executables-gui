<script lang="ts" context="module">
    import { writable } from 'svelte/store'

    export const shouldRemoveNACalculations = writable<boolean>(false)
    export const toleranceMultiplier = writable<number>(2.0)

</script>


<script lang="ts">
    import * as Card from '$lib/components/ui/card'
    import { Icon, InformationCircle } from 'svelte-hero-icons'
    import { Switch } from '$lib/components/ui/switch'
    import { Label } from '$lib/components/ui/label'
    import * as Tooltip from '$lib/components/ui/tooltip'
    import { Input } from '$lib/components/ui/input'
</script>

<Card.Root class="col-span-1 md:col-span-2">
    <Card.Header>
        <Card.Title>Settings</Card.Title>
    </Card.Header>
    <Card.Content class="flex flex-col space-y-4">
        <div class="flex flex-col space-y-1.5">
            <Label class="flex items-center gap-2" for="tolerance-multiplier">Tolerance Multiplier
                <Tooltip.Root openDelay={0}>
                    <Tooltip.Trigger>
                        <Icon src={InformationCircle} mini class='w-4 h-4' />
                    </Tooltip.Trigger>
                    <Tooltip.Content class='max-w-md'>
                        <p>SRM heuristically detects data pertaining to different charges for a given peptide. It uses
                            standard deviation times this multipler to determine which mass-charge ratio don't belong in the
                            same group. Adjust to what works, but 2.0 is recommended.</p>
                    </Tooltip.Content>
                </Tooltip.Root>
            </Label>
            <Input id="tolerance-multiplier" type="number" step=".1" bind:value={$toleranceMultiplier} />
        </div>
        <div class="flex items-center space-x-2">
            <Switch id="should-remove-na-calculations" bind:checked={$shouldRemoveNACalculations} />
            <Label for="should-remove-na-calculations">Avoid N/A Calculations</Label>
            <Tooltip.Root openDelay={0}>
                <Tooltip.Trigger>
                    <Icon src={InformationCircle} mini class='w-4 h-4' />
                </Tooltip.Trigger>
                <Tooltip.Content class='max-w-md'>
                    <p>Removes samples with no detected peaks from the input for a given peptide to avoid "-nan(ind)"
                        calculations.</p>
                </Tooltip.Content>
            </Tooltip.Root>
        </div>
    </Card.Content>
</Card.Root>