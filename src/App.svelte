<script lang="ts">
  import './styles.css'
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
  <h1 class="text-xl">Hi</h1>
  <div class="mockup-code">
    <pre><code>{JSON.stringify(error)}</code></pre>
  </div>
</main>

<style>
</style>
