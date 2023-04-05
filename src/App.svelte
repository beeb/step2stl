<script lang="ts">
  import './styles.css'
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api/tauri'
  import { open } from '@tauri-apps/api/dialog'
  import { default as toast, Toaster } from 'svelte-french-toast'

  const handleClick = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'STEP',
            extensions: ['step', 'stp']
          }
        ]
      })
      if (selected) {
        await convert(selected)
      }
    } catch (e) {
      toast.error(e.message)
    }
    toast.success('Conversion complete')
  }

  const convert = async (path) => {
    const extension = path.split('.').pop()
    if (extension.toLowerCase() !== 'step') {
      toast.error('Only STEP files are supported')
      return
    }
    try {
      await invoke('convert', { path, chordError: 0.005, angleRes: 1.0 })
    } catch (e) {
      toast.error(e.message)
    }
  }

  listen('tauri://file-drop', async (event) => {
    const path = event.payload[0]
    await convert(path)
  })
</script>

<main class="p-6 w-full h-screen flex flex-col gap-6">
  <h1 class="text-xl text-center">STEP to STL converter</h1>
  <div class="card bg-base-300 shadow-xl flex-grow">
    <button type="button" class="card-body flex items-center justify-center cursor-pointer" on:click={handleClick}>
      <div>Drop a file here</div>
      <div>or click to select a file</div>
    </button>
  </div>
</main>

<Toaster containerClassName="toast-container" />

<style>
</style>
