<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  let selectedShow: string | null = null;
  
  let shows: string[] = [];
  
  async function getShows() {
    const result = await invoke("get_shows");
    shows = result as string[];
  }
  onMount(getShows);
</script>
{#if !selectedShow}
<div class="library-container">
  <h2>All Shows ({shows.length})</h2>
  
  <div class="show-grid">
    {#each shows as show (show)}
      <div 
        role="button" 
        tabindex="0"
        class="show-card" 
        on:click={() => selectedShow = show}
        on:keydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            selectedShow = show;
            e.preventDefault();
          }
        }}
      >
        <div class="cover-wrapper">
          <div class="overlay">
            <h1 class="show-title">{show}</h1>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>
<style>
  :global(body) {
    background-color: transparent;
    color: #c7d5e0;
    font-family: "Motiva Sans", Sans-Serif, Arial;
    margin: 0;
    padding: 20px;
  }

  .library-container {
    max-width: 1400px;
    margin: 10px auto;
  }

  h2 {
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: 1px;
    color: #ffffff;
    margin-bottom: 20px;
    padding-bottom: 8px;
  }
  .show-grid {
    display: grid;
    /* Automatically fits cards, scaling down to 180px minimum */
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 24px;
  }

  .show-card {
    display: flex;
    flex-direction: column;
    cursor: pointer;
    transition: transform 0.2s ease, filter 0.2s ease;
  }

  .cover-wrapper {
    position: relative;
    width: 100%;
    aspect-ratio: 2 / 3;
    border-radius: 4px;
    overflow: hidden;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    background-color:transparent;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .show-card:hover {
    transform: translateY(-4px);
  }

  .show-card:hover .cover-wrapper {
    box-shadow: 0 8px 24px rgba(102, 192, 244, 0.2);
  }

  .overlay {
    position: absolute;
    inset: 0;
    background: linear-gradient(to top, rgba(0,0,0,0.8) 0%, rgba(0,0,0,0) 50%);
    display: flex;
    justify-content: center;
    align-items: center;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .show-card:hover .overlay {
    opacity: 1;
  }

  .show-title {
    font-size: 2rem;
    font-weight: 500;
    color: #d6d7d8;
    overflow: hidden;
    margin: auto;
    text-align: center;
  }

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>

{/if}
{#if selectedShow}
	<button on:click={() => selectedShow = null}>
		Close
	</button>
	<style>
  :global(body) {
    background-color: transparent;
    color: #c7d5e0;
    font-family: "Motiva Sans", Sans-Serif, Arial;
    margin: 0;
    padding: 20px;
  }



@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>

{/if}

