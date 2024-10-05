<script lang="ts">
  import './styles.css'
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api/core'
  import { open } from '@tauri-apps/plugin-dialog'
  import { default as toast, Toaster } from 'svelte-french-toast'

  const qualityValues = {
    fdm: {
      chordError: 0.005,
      angleRes: 1.0
    },
    sla: {
      chordError: 0.002,
      angleRes: 0.5
    },
    render: {
      chordError: 0.001,
      angleRes: 0.3
    }
  }

  let quality: keyof typeof qualityValues = 'fdm'
  let binary = false

  const handleClick = async () => {
    try {
      const selected = (await open({
        multiple: false,
        filters: [
          {
            name: 'STEP',
            extensions: ['step', 'stp', 'STEP', 'STP']
          }
        ]
      })) as string
      if (selected) {
        await convert(selected)
      }
    } catch (e) {
      let message = String(e)
      if (e instanceof Error) {
        message = e.message
      }
      toast.error(message)
    }
  }

  const convert = async (path: string) => {
    const extension = path.split('.').pop() ?? ''
    if (!['step', 'stp'].includes(extension.toLowerCase())) {
      toast.error('Only STEP files are supported')
      return
    }
    toast.promise(
      invoke<string>('convert', {
        path,
        chordError: qualityValues[quality].chordError,
        angleRes: qualityValues[quality].angleRes,
        binary
      }),
      {
        loading: 'Converting...',
        success: (stlPath: string) => {
          const filename = stlPath.replace(/^.*[\\\/]/, '')
          return `File converted: ${filename}`
        },
        error: 'Conversion failed'
      },
      {
        duration: 3000
      }
    )
  }

  listen<string>('tauri://file-drop', async (event) => {
    const path = event.payload[0]
    await convert(path)
  })
</script>

<main class="p-6 w-full h-screen flex flex-col gap-6">
  <div class="form-control flex-row gap-2">
    <label class="label" for="quality">
      <span class="label-text">Quality:</span>
    </label>
    <select id="quality" class="select select-bordered grow" bind:value={quality}>
      <option value="fdm">FDM Printer</option>
      <option value="sla">SLA Printer</option>
      <option value="render">3D Render</option>
    </select>
    <label class="label cursor-pointer gap-2">
      <span class="label-text">Binary:</span>
      <input type="checkbox" bind:checked={binary} class="checkbox" />
    </label>
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
