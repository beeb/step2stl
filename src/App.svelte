<script lang="ts">
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api/tauri'

  let error = {}

  listen('tauri://file-drop', async (event) => {
    const path = event.payload[0]
    try {
      await invoke('convert', { path, chordError: 0.005, angleRes: 1.0 })
    } catch (e) {
      error = e
    }
  })
</script>

<main class="container">
  {JSON.stringify(error)}
</main>

<style>
</style>
