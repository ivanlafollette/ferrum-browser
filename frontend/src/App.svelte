<script>
  import { onMount } from 'svelte';
  // Array of tab objects: each has an id, url and title
  let tabs = [
    { id: 0, url: 'https://duckduckgo.com', title: 'DuckDuckGo' }
  ];
  // Index of the currently active tab
  let active = 0;

  // Add a new blank tab and make it active
  function addTab() {
    tabs = [...tabs, { id: Date.now(), url: '', title: 'New Tab' }];
    active = tabs.length - 1;
  }

  // Close the tab at the given index and adjust active index
  function closeTab(index) {
    tabs = tabs.filter((_, i) => i !== index);
    if (active >= tabs.length) {
      active = tabs.length - 1;
    }
  }

  // Switch the currently active tab
  function switchTab(index) {
    active = index;
  }

  // Navigate to the URL in the active tab; update title to the URL for now
  function navigate(index) {
    // When the iframe loads we could extract the title, but for the MVP we simply set title to URL
    tabs[index].title = tabs[index].url;
  }
</script>

<!-- Tab bar across the top -->
<div class="tab-bar">
  {#each tabs as tab, i}
    <div class="tab {i === active ? 'active' : ''}" on:click={() => switchTab(i)}>
      <span class="tab-title">{tab.title || 'New Tab'}</span>
      <button class="close-button" on:click|stopPropagation={() => closeTab(i)}>\u2715</button>
    </div>
  {/each}
  <button class="add-tab" on:click={addTab}>+</button>
</div>

{#if tabs[active]}
<div class="browser">
  <div class="url-bar">
    <input type="text" bind:value={tabs[active].url} placeholder="Enter URL and hit Enter" on:keydown={(e) => { if (e.key === 'Enter') navigate(active) }} />
    <button on:click={() => navigate(active)}>Go</button>
  </div>
  <iframe
    class="view"
    src={tabs[active].url}
    sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
  ></iframe>
</div>
{/if}

<style>
  .tab-bar {
    display: flex;
    align-items: center;
    background-color: #333;
    color: #f5f5f5;
    padding: 0.2rem;
  }
  .tab {
    display: flex;
    align-items: center;
    padding: 0.4rem 0.8rem;
    margin-right: 0.2rem;
    background-color: #444;
    border-radius: 4px 4px 0 0;
    cursor: pointer;
  }
  .tab.active {
    background-color: #555;
    font-weight: bold;
  }
  .tab-title {
    margin-right: 0.4rem;
  }
  .close-button {
    background: none;
    border: none;
    color: #ccc;
    cursor: pointer;
    font-size: 0.8rem;
  }
  .close-button:hover {
    color: #fff;
  }
  .add-tab {
    background-color: #444;
    border: none;
    color: #ddd;
    padding: 0.4rem 0.8rem;
    font-size: 1.2rem;
    cursor: pointer;
    border-radius: 4px;
  }
  .browser {
    display: flex;
    flex-direction: column;
    height: calc(100vh - 40px);
  }
  .url-bar {
    display: flex;
    padding: 0.5rem;
    background-color: #222;
  }
  .url-bar input {
    flex: 1;
    padding: 0.4rem;
    margin-right: 0.5rem;
    border: 1px solid #555;
    border-radius: 4px;
    background-color: #111;
    color: #f5f5f5;
  }
  .url-bar button {
    padding: 0.4rem 0.8rem;
    background-color: #444;
    border: none;
    color: #fff;
    border-radius: 4px;
    cursor: pointer;
  }
  .view {
    flex: 1;
    border: none;
  }
</style>
