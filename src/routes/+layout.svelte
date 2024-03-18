<script>
    import { Toaster } from 'svelte-sonner'
    import '$lib/assets/css/cal-sans.css'
    import '@fontsource-variable/inter'

    import '../app.css'
</script>

<Toaster richColors />
<slot />