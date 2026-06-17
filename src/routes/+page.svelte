<script>
  // Sample data simulating a Steam library
  let shows = [
    { id: 1, title: "Cyberpunk 2077", cover: "https://via.placeholder.com/300x450/111/fff?text=Cyberpunk" },
    { id: 2, title: "Elden Ring", cover: "https://via.placeholder.com/300x450/222/fff?text=Elden+Ring" },
    { id: 3, title: "Hades II", cover: "https://via.placeholder.com/300x450/333/fff?text=Hades+II" },
    { id: 4, title: "Witcher 3", cover: "https://via.placeholder.com/300x450/444/fff?text=Witcher+3" },
    { id: 5, title: "Balatro", cover: "https://via.placeholder.com/300x450/555/fff?text=Balatro" },
    { id: 6, title: "Stardew Valley", cover: "https://via.placeholder.com/300x450/666/fff?text=Stardew" },
  ];
</script>

<div class="library-container">
  <h2>All Shows ({shows.length})</h2>
  
  <div class="show-grid">
    {#each shows as show (show.id)}
      <div class="show-card">
        <div class="cover-wrapper">
          <div class="overlay">
            <button class="play-btn">▶</button>
            <h1>{show.title}</h1>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  /* Dark Steam-like background */
  :global(body) {
    background-color: transparent;
    color: #c7d5e0;
    font-family: "Motiva Sans", Sans-Serif, Arial;
    margin: 0;
    padding: 20px;
  }

  .library-container {
    max-width: 1400px;
    margin: 0 auto;
  }

  h2 {
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: 1px;
    color: #ffffff;
    margin-bottom: 20px;
    padding-bottom: 8px;
  }

  /* The Steam Grid Layout */
  .show-grid {
    display: grid;
    /* Automatically fits cards, scaling down to 180px minimum */
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 24px;
  }

  /* Game Card Styling */
  .show-card {
    display: flex;
    flex-direction: column;
    cursor: pointer;
    transition: transform 0.2s ease, filter 0.2s ease;
  }

  /* Aspect ratio box to keep standard Steam vertical cover dimensions (2:3) */
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

  .cover-wrapper img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  /* Hover Effects */
  .show-card:hover {
    transform: translateY(-4px);
  }

  .show-card:hover .cover-wrapper {
    box-shadow: 0 8px 24px rgba(102, 192, 244, 0.2);
  }

  /* Hover Play Button Overlay */
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

  .play-btn {
    background: #5c7e10;
    color: white;
    border: none;
    border-radius: 50%;
    width: 50px;
    height: 50px;
    font-size: 20px;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    text-shadow: 0 1px 2px rgba(0,0,0,0.5);
    box-shadow: 0 4px 10px rgba(0,0,0,0.4);
    transition: background 0.1s;
    padding-left: 4px; /* Optical alignment for play arrow */
  }

  .play-btn:hover {
    background: #78a317;
    transform: scale(1.05);
  }

  /* Title Info text underneath */
  .game-info {
    margin-top: 8px;
    padding: 2px;
  }

  .game-title {
    font-size: 13px;
    font-weight: 500;
    color: #d6d7d8;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: block;
  }

  .game-card:hover .game-title {
    color: #ffffff;
  }

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
