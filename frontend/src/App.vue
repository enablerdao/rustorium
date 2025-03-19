<template>
  <div id="app">
    <header class="header">
      <div class="logo">
        <img src="./assets/logo.svg" alt="Rustorium Logo" />
        <h1>Rustorium</h1>
      </div>
      <nav class="nav">
        <a href="#" class="nav-link active">Dashboard</a>
        <a href="#" class="nav-link">Explorer</a>
        <a href="#" class="nav-link">Wallet</a>
        <a href="#" class="nav-link">Contracts</a>
        <a href="#" class="nav-link">Docs</a>
      </nav>
      <div class="user-menu">
        <button class="theme-toggle" @click="toggleTheme">
          <i class="icon" :class="isDarkTheme ? 'icon-sun' : 'icon-moon'"></i>
        </button>
        <div class="network-status" :class="{ 'network-status-online': isOnline }">
          <span class="status-dot"></span>
          <span class="status-text">{{ isOnline ? 'Connected' : 'Disconnected' }}</span>
        </div>
      </div>
    </header>
    
    <main class="main">
      <div class="container">
        <h2 class="section-title">Rustorium Terminal</h2>
        <Terminal />
      </div>
    </main>
    
    <footer class="footer">
      <div class="container">
        <p>&copy; 2023 Rustorium. All rights reserved.</p>
        <div class="footer-links">
          <a href="#">GitHub</a>
          <a href="#">Documentation</a>
          <a href="#">Community</a>
        </div>
      </div>
    </footer>
  </div>
</template>

<script>
import Terminal from './components/Terminal.vue';

export default {
  name: 'App',
  components: {
    Terminal
  },
  data() {
    return {
      isDarkTheme: true,
      isOnline: true
    };
  },
  methods: {
    toggleTheme() {
      this.isDarkTheme = !this.isDarkTheme;
      document.body.classList.toggle('light-theme', !this.isDarkTheme);
    }
  },
  mounted() {
    // Set initial theme
    document.body.classList.toggle('light-theme', !this.isDarkTheme);
    
    // Check network status periodically
    setInterval(() => {
      this.isOnline = navigator.onLine;
    }, 5000);
  }
};
</script>

<style>
:root {
  --primary-color: #00bcd4;
  --secondary-color: #4caf50;
  --background-color: #121212;
  --surface-color: #1e1e1e;
  --text-color: #f0f0f0;
  --text-secondary-color: #a0a0a0;
  --border-color: #333;
  --error-color: #f44336;
  --warning-color: #ffeb3b;
  --success-color: #4caf50;
}

body.light-theme {
  --background-color: #f5f5f5;
  --surface-color: #ffffff;
  --text-color: #212121;
  --text-secondary-color: #757575;
  --border-color: #e0e0e0;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: 'Roboto', sans-serif;
  background-color: var(--background-color);
  color: var(--text-color);
  line-height: 1.6;
}

#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.container {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}

.header {
  background-color: var(--surface-color);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 16px 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  display: flex;
  align-items: center;
  padding: 0 20px;
}

.logo img {
  height: 40px;
  margin-right: 16px;
}

.logo h1 {
  font-size: 24px;
  font-weight: 500;
  color: var(--primary-color);
}

.nav {
  display: flex;
  gap: 24px;
}

.nav-link {
  color: var(--text-color);
  text-decoration: none;
  font-weight: 500;
  padding: 8px 0;
  position: relative;
}

.nav-link:hover {
  color: var(--primary-color);
}

.nav-link.active {
  color: var(--primary-color);
}

.nav-link.active::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 2px;
  background-color: var(--primary-color);
}

.user-menu {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 0 20px;
}

.theme-toggle {
  background: none;
  border: none;
  color: var(--text-color);
  cursor: pointer;
  font-size: 20px;
}

.network-status {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--error-color);
}

.network-status-online {
  color: var(--success-color);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: var(--error-color);
}

.network-status-online .status-dot {
  background-color: var(--success-color);
}

.main {
  flex: 1;
  padding: 40px 0;
}

.section-title {
  font-size: 24px;
  margin-bottom: 24px;
  color: var(--primary-color);
}

.footer {
  background-color: var(--surface-color);
  padding: 24px 0;
  margin-top: auto;
}

.footer .container {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.footer-links {
  display: flex;
  gap: 24px;
}

.footer-links a {
  color: var(--text-secondary-color);
  text-decoration: none;
}

.footer-links a:hover {
  color: var(--primary-color);
}

/* Icons */
.icon {
  font-style: normal;
}

.icon-sun::before {
  content: "☀️";
}

.icon-moon::before {
  content: "🌙";
}

@media (max-width: 768px) {
  .header {
    flex-direction: column;
    gap: 16px;
  }
  
  .nav {
    overflow-x: auto;
    width: 100%;
    padding: 0 20px;
  }
  
  .footer .container {
    flex-direction: column;
    gap: 16px;
  }
}
</style>