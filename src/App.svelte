<script lang="ts">
  import "./styles.css";
  import { getCurrentWebview } from "@tauri-apps/api/webview";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { default as toast, Toaster } from "svelte-french-toast";

  let unlisten: () => void | undefined;

  const qualityValues = {
    fdm: {
      chordError: 0.005,
      angleRes: 1.0,
    },
    sla: {
      chordError: 0.002,
      angleRes: 0.5,
    },
    render: {
      chordError: 0.001,
      angleRes: 0.3,
    },
  } as const;

  let quality = $state<keyof typeof qualityValues>("fdm");
  let binary = $state(false);

  const handleClick = async () => {
    try {
      const selected = (await open({
        multiple: false,
        filters: [
          {
            name: "STEP",
            extensions: ["step", "stp", "STEP", "STP"],
          },
        ],
      })) as string;
      if (selected) {
        await convert(selected);
      }
    } catch (e) {
      let message = String(e);
      if (e instanceof Error) {
        message = e.message;
      }
      toast.error(message);
    }
  };

  const convert = async (path: string) => {
    const extension = path.split(".").pop() ?? "";
    if (!["step", "stp"].includes(extension.toLowerCase())) {
      toast.error("Only STEP files are supported");
      return;
    }
    toast.promise(
      invoke<string>("convert", {
        path,
        chordError: qualityValues[quality].chordError,
        angleRes: qualityValues[quality].angleRes,
        binary,
      }),
      {
        loading: "Converting...",
        success: (stlPath: string) => {
          const filename = stlPath.replace(/^.*[\\\/]/, "");
          return `File converted: ${filename}`;
        },
        error: "Conversion failed",
      },
      {
        duration: 3000,
      }
    );
  };

  $effect(() => {
    (async () => {
      unlisten = await getCurrentWebview().onDragDropEvent(async (event) => {
        if (event.payload.type === "drop") {
          for (const path of event.payload.paths) {
            await convert(path);
          }
        }
      });
    })();
    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  });
</script>

<main class="p-6 w-full h-screen flex flex-col gap-6">
  <div class="w-full flex items-center gap-4">
    <label class="select grow">
      <span class="label">Quality</span>
      <select id="quality" bind:value={quality}>
        <option value="fdm">FDM Printer</option>
        <option value="sla">SLA Printer</option>
        <option value="render">3D Render</option>
      </select>
    </label>
    <label class="flex flex-nowrap shrink-0 gap-2 cursor-pointer">
      <input type="checkbox" bind:checked={binary} class="checkbox" />
      Binary
    </label>
  </div>
  <div class="card bg-base-300 shadow-xl grow">
    <button
      type="button"
      class="card-body flex items-center justify-center cursor-pointer"
      onclick={handleClick}
    >
      <div>Drop a file here</div>
      <div>or click to select a file</div>
    </button>
  </div>
</main>

<Toaster containerClassName="toast-container" />

<style>
</style>
