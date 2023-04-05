<script lang="ts">
  import './styles.css'
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api/tauri'
  import { open } from '@tauri-apps/api/dialog'
  import { default as toast, Toaster } from 'svelte-french-toast'

  let quality = 'fdm'

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
  }

  const convert = async (path) => {
    const extension = path.split('.').pop()
    if (extension.toLowerCase() !== 'step') {
      toast.error('Only STEP files are supported')
      return
    }
    try {
      await invoke('convert', { path, chordError: 0.005, angleRes: 1.0 })
      toast.success('Conversion complete')
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
  <div class="form-control flex-row gap-4">
    <label class="label" for="quality">
      <span class="label-text">Quality:</span>
    </label>
    <select id="quality" class="select select-bordered grow" bind:value={quality}>
      <option value="fdm">FDM Printer</option>
      <option value="sla">SLA Printer</option>
      <option value="render">3D Render</option>
    </select>
  </div>
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
