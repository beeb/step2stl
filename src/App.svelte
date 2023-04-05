<script lang="ts">
  import './styles.css'
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api/tauri'

  let error = {}

  listen('tauri://file-drop', async (event) => {
    const path = event.payload[0]
    const extension = path.split('.').pop()
    if (extension.toLowerCase() !== 'step') {
      error = { message: 'Only STEP files are supported' }
      return
    }
    try {
      await invoke('convert', { path, chordError: 0.005, angleRes: 1.0 })
    } catch (e) {
      error = e
    }
  })
</script>

<main class="p-6 w-full h-screen flex flex-col gap-6">
  <h1 class="text-xl text-center">STEP to STL converter</h1>
  <div class="card bg-base-300 shadow-xl flex-grow">
    <div class="card-body flex items-center justify-center">Drop a file here to convert</div>
  </div>
</main>

<style>
</style>
