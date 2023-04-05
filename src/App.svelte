<script lang="ts">
  import { listen } from '@tauri-apps/api/event'

  listen('tauri://file-drop', (event) => {
    const path = event.payload[0]
  })
</script>

<main class="container">Hi</main>

<style>
</style>
