<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import {listen} from "@tauri-apps/api/event";
    import {info} from '@tauri-apps/plugin-log';

    let foundDevices: Array<unknown> = $state([]);

    async function handlePlusButton() {
        await invoke('start_mdns_scan');
    }

    onMount(() => {
        const unlisten = listen('device-found', (event) => {
            info(`Device found: ${event.payload}`);
            foundDevices = [...foundDevices, event.payload];
        });

        return () => {
            unlisten.then(f => f());
        };
    });
</script>

<main class="container">
    <button onclick={handlePlusButton}>Listen</button>
    <ul>
        {#each foundDevices as device (device)}
            <li>{device}</li>
        {/each}
    </ul>
</main>

<style>
</style>
