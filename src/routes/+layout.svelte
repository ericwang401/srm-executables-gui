<script>
    import { Toaster } from 'svelte-sonner'
    import '$lib/assets/css/cal-sans.css'
    import '@fontsource-variable/inter'
    import ContentContainer from '$lib/components/ui/ContentContainer.svelte'
    import Spinner from '$lib/components/ui/Spinner.svelte'
    import { getVersion } from '@tauri-apps/api/app'

    import '../app.css'
</script>

<Toaster richColors />
<div class="h-2 w-full bg-gradient-to-r from-indigo-600 to-fuchsia-400" />

<ContentContainer>
    <div class="pb-6 pt-4 lg:pt-8">
        <h1 class="font-heading text-lg font-semibold md:text-4xl">
            SRM Executables
            {#await getVersion()}
                <Spinner class="inline h-4 w-4" />
            {:then version}
                <span class="text-sm font-normal text-slate-500">v{version}</span>
            {/await}
        </h1>
        <p class="max-w-2xl text-muted-foreground text-sm md:text-base">
            A utility that calculates the synthesis rates of proteins. Created by <a
            class="link"
            href="https://github.com/ericwang401"
            target="_blank"
        >
            Eric Wang
        </a> | <a
            class="link"
            href="https://github.com/OMRF/srm-executables-gui"
            target="_blank"
        >
            Source
        </a>
        </p>
    </div>

    <slot />
</ContentContainer>

<style lang="postcss">
    .link {
        @apply font-medium underline underline-offset-4;
    }
</style>