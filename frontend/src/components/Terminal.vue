<template>
  <div class="terminal-container">
    <div class="terminal-header">
      <div class="terminal-title">Rustorium Terminal</div>
      <div class="terminal-controls">
        <button class="terminal-button" @click="clearTerminal">Clear</button>
        <button class="terminal-button" @click="toggleFullscreen">
          {{ isFullscreen ? 'Exit Fullscreen' : 'Fullscreen' }}
        </button>
      </div>
    </div>
    <div ref="terminalOutput" class="terminal-output" @click="focusInput">
      <div v-if="showLogo" class="terminal-logo">
        <pre v-html="logo"></pre>
      </div>
      <div v-for="(line, index) in outputLines" :key="index" class="terminal-line" v-html="line"></div>
    </div>
    <div class="terminal-input-container">
      <span class="terminal-prompt">{{ prompt }}</span>
      <input
        ref="terminalInput"
        v-model="inputText"
        class="terminal-input"
        @keydown.enter="executeCommand"
        @keydown.up="navigateHistory(-1)"
        @keydown.down="navigateHistory(1)"
        @keydown.tab.prevent="autocomplete"
      />
    </div>
  </div>
</template>

<script>
export default {
  name: 'Terminal',
  data() {
    return {
      outputLines: [],
      inputText: '',
      commandHistory: [],
      historyIndex: -1,
      isFullscreen: false,
      showLogo: true,
      currentAccount: null,
      networkStatus: {
        chainId: 1337,
        currentBlock: 1234567,
        syncStatus: 'Fully Synced',
        syncPercentage: 100,
        peers: 25,
        tps: 156.3,
        gasPrice: 5.2
      },
      nodeStats: {
        cpuUsage: 23,
        memoryUsed: '1.2GB',
        memoryTotal: '8GB',
        diskUsed: '34.5GB',
        uptime: '5d 12h 34m',
        lastBlockTime: '3s ago'
      },
      logo: `
╭───────────────────────────────────────────────────────────────╮
│                                                               │
│   <span class="cyan">██████╗ ██╗   ██╗███████╗████████╗ ██████╗ ██████╗ ██╗██╗</span>   │
│   <span class="cyan">██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██║██║</span>   │
│   <span class="cyan">██████╔╝██║   ██║███████╗   ██║   ██║   ██║██████╔╝██║██║</span>   │
│   <span class="cyan">██╔══██╗██║   ██║╚════██║   ██║   ██║   ██║██╔══██╗██║██║</span>   │
│   <span class="cyan">██║  ██║╚██████╔╝███████║   ██║   ╚██████╔╝██║  ██║██║███████</span>│
│   <span class="cyan">╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝╚══════╝</span>│
│                                                <span class="cyan">v1.0.0</span>         │
╰───────────────────────────────────────────────────────────────╯
`
    };
  },
  computed: {
    prompt() {
      return this.currentAccount ? `${this.currentAccount}> ` : '> ';
    }
  },
  mounted() {
    this.focusInput();
    this.initializeTerminal();
  },
  methods: {
    initializeTerminal() {
      this.printLine('<span class="green">[INFO]</span> Rustorium node starting...');
      this.printLine('<span class="green">[INFO]</span> Loading configuration from config.toml');
      this.printLine('<span class="green">[INFO]</span> Database initialized');
      this.printLine('<span class="green">[INFO]</span> P2P network initialized');
      this.printLine('<span class="green">[INFO]</span> API server listening on 127.0.0.1:50128');
      this.printLine('<span class="green">[INFO]</span> WebSocket server listening on 127.0.0.1:50129');
      this.printLine('');
      
      this.printNetworkStatus();
      this.printNodeStats();
      
      this.printLine('\nRustorium CLI ready. Type <span class="cyan">help</span> for available commands.');
    },
    
    printNetworkStatus() {
      const { chainId, currentBlock, syncStatus, syncPercentage, peers, tps, gasPrice } = this.networkStatus;
      
      this.printLine('╭─ <span class="cyan bold">NETWORK STATUS</span> ───────────────────────────────────────────────╮');
      this.printLine(`│ <span class="cyan">Chain ID:</span> ${chainId}                                                 │`);
      this.printLine(`│ <span class="cyan">Current Block:</span> #${currentBlock}                                      │`);
      this.printLine(`│ <span class="cyan">Sync Status:</span> ${syncPercentage}% (${syncStatus})                      │`);
      this.printLine(`│ <span class="cyan">Peers:</span> ${peers} connected                                            │`);
      this.printLine(`│ <span class="cyan">TPS:</span> ${tps}                                                          │`);
      this.printLine(`│ <span class="cyan">Gas Price:</span> ${gasPrice} Gwei                                          │`);
      this.printLine('╰───────────────────────────────────────────────────────────────╯');
    },
    
    printNodeStats() {
      const { cpuUsage, memoryUsed, memoryTotal, diskUsed, uptime, lastBlockTime } = this.nodeStats;
      
      this.printLine('╭─ <span class="cyan bold">NODE STATS</span> ─────────────────────────────────────────────────╮');
      this.printLine(`│ <span class="cyan">CPU: ${cpuUsage}%</span>  │ <span class="cyan">Memory: ${memoryUsed}/${memoryTotal}</span>  │ <span class="cyan">Disk: ${diskUsed} used</span>           │`);
      this.printLine(`│ <span class="cyan">Uptime: ${uptime}</span>             │ <span class="cyan">Last Block: ${lastBlockTime}</span>           │`);
      this.printLine('╰───────────────────────────────────────────────────────────────╯');
    },
    
    executeCommand() {
      const command = this.inputText.trim();
      if (command) {
        this.printLine(`${this.prompt}${command}`);
        this.commandHistory.push(command);
        this.historyIndex = this.commandHistory.length;
        
        this.processCommand(command);
        
        this.inputText = '';
        this.$nextTick(() => {
          this.scrollToBottom();
        });
      }
    },
    
    processCommand(command) {
      const parts = command.split(' ');
      const cmd = parts[0].toLowerCase();
      const args = parts.slice(1);
      
      switch (cmd) {
        case 'help':
          this.showHelp(args[0]);
          break;
        case 'clear':
        case 'cls':
          this.clearTerminal();
          break;
        case 'exit':
        case 'quit':
          this.printLine('Cannot exit from browser terminal. Use the browser controls instead.');
          break;
        case 'account':
          this.handleAccountCommand(args);
          break;
        case 'block':
          this.handleBlockCommand(args);
          break;
        case 'contract':
          this.handleContractCommand(args);
          break;
        case 'network':
          this.handleNetworkCommand(args);
          break;
        case 'token':
          this.handleTokenCommand(args);
          break;
        case 'tx':
          this.handleTxCommand(args);
          break;
        case 'system':
          this.handleSystemCommand(args);
          break;
        case 'config':
          this.handleConfigCommand(args);
          break;
        case 'debug':
          this.handleDebugCommand(args);
          break;
        default:
          this.printLine(`<span class="red">Error:</span> Unknown command '${cmd}'`);
          this.printLine("Type <span class=\"cyan\">help</span> for available commands.");
      }
    },
    
    showHelp(command) {
      if (!command) {
        this.printLine('Available commands:');
        this.printLine('  <span class="cyan">account</span> - Manage accounts and wallets');
        this.printLine('  <span class="cyan">block</span> - View block information');
        this.printLine('  <span class="cyan">contract</span> - Deploy and interact with smart contracts');
        this.printLine('  <span class="cyan">network</span> - View and configure network settings');
        this.printLine('  <span class="cyan">token</span> - Manage tokens (ERC-20/ERC-721)');
        this.printLine('  <span class="cyan">tx</span> - Create and manage transactions');
        this.printLine('  <span class="cyan">system</span> - System and node management');
        this.printLine('  <span class="cyan">config</span> - Configure node settings');
        this.printLine('  <span class="cyan">debug</span> - Debugging tools');
        this.printLine('  <span class="cyan">clear</span> - Clear the screen');
        this.printLine('  <span class="cyan">exit</span> - Exit the CLI (not functional in browser)');
        this.printLine('\nType <span class="cyan">help &lt;command&gt;</span> for more information on a specific command.');
      } else {
        switch (command) {
          case 'account':
            this.printLine('Account commands:');
            this.printLine('  <span class="cyan">get</span> &lt;address&gt; - Get account by address');
            this.printLine('  <span class="cyan">create</span> - Create a new account');
            this.printLine('  <span class="cyan">list</span> [limit] [offset] - List accounts');
            this.printLine('  <span class="cyan">use</span> &lt;address&gt; - Set current account');
            break;
          case 'block':
            this.printLine('Block commands:');
            this.printLine('  <span class="cyan">get</span> &lt;id&gt; - Get block by number or hash');
            this.printLine('  <span class="cyan">latest</span> - Get latest block');
            this.printLine('  <span class="cyan">list</span> [limit] [offset] - List blocks');
            break;
          case 'contract':
            this.printLine('Contract commands:');
            this.printLine('  <span class="cyan">get</span> &lt;address&gt; - Get contract by address');
            this.printLine('  <span class="cyan">deploy</span> &lt;file&gt; - Deploy contract from file');
            this.printLine('  <span class="cyan">call</span> &lt;address&gt; &lt;method&gt; [args] - Call contract method');
            this.printLine('  <span class="cyan">list</span> [limit] [offset] - List contracts');
            break;
          case 'network':
            this.printLine('Network commands:');
            this.printLine('  <span class="cyan">status</span> - Show network status');
            this.printLine('  <span class="cyan">peers</span> - List connected peers');
            this.printLine('  <span class="cyan">sync</span> - Show sync status');
            break;
          case 'token':
            this.printLine('Token commands:');
            this.printLine('  <span class="cyan">create</span> &lt;name&gt; &lt;symbol&gt; &lt;type&gt; [supply] - Create token');
            this.printLine('  <span class="cyan">get</span> &lt;address&gt; - Get token by address');
            this.printLine('  <span class="cyan">list</span> [limit] [offset] - List tokens');
            this.printLine('  <span class="cyan">balance</span> &lt;token&gt; &lt;account&gt; - Get token balance');
            this.printLine('  <span class="cyan">transfer</span> &lt;token&gt; &lt;to&gt; &lt;amount&gt; - Transfer tokens');
            break;
          case 'tx':
            this.printLine('Transaction commands:');
            this.printLine('  <span class="cyan">get</span> &lt;id&gt; - Get transaction by ID');
            this.printLine('  <span class="cyan">send</span> &lt;to&gt; &lt;value&gt; - Send transaction');
            this.printLine('  <span class="cyan">list</span> [limit] [offset] - List transactions');
            break;
          case 'system':
            this.printLine('System commands:');
            this.printLine('  <span class="cyan">stats</span> - Show system stats');
            this.printLine('  <span class="cyan">version</span> - Show version information');
            this.printLine('  <span class="cyan">uptime</span> - Show node uptime');
            break;
          case 'config':
            this.printLine('Config commands:');
            this.printLine('  <span class="cyan">get</span> &lt;key&gt; - Get config value');
            this.printLine('  <span class="cyan">set</span> &lt;key&gt; &lt;value&gt; - Set config value');
            this.printLine('  <span class="cyan">list</span> - List all config values');
            break;
          case 'debug':
            this.printLine('Debug commands:');
            this.printLine('  <span class="cyan">trace-tx</span> &lt;id&gt; - Trace transaction execution');
            this.printLine('  <span class="cyan">logs</span> - Show debug logs');
            this.printLine('  <span class="cyan">metrics</span> - Show performance metrics');
            break;
          default:
            this.printLine(`No help available for '${command}'`);
        }
      }
    },
    
    handleAccountCommand(args) {
      if (args.length === 0) {
        this.showHelp('account');
        return;
      }
      
      const subcommand = args[0];
      
      switch (subcommand) {
        case 'get':
          if (args.length < 2) {
            this.printLine('Usage: account get <address>');
            return;
          }
          const address = args[1];
          this.printLine(`Getting account ${address}...`);
          // Simulate API call
          setTimeout(() => {
            this.printLine(`Address: <span class="green">${address}</span>`);
            this.printLine(`Balance: <span class="yellow">100.0 ETH</span>`);
            this.printLine('Nonce: 5');
            this.printLine('Type: EOA');
            this.printLine('Created At: 2023-06-15 14:32:45 UTC');
          }, 500);
          break;
        case 'create':
          this.printLine('Creating new account...');
          // Simulate API call
          setTimeout(() => {
            const newAddress = `0x${Math.random().toString(16).substr(2, 40)}`;
            this.printLine('Account created successfully:');
            this.printLine(`Address: <span class="green">${newAddress}</span>`);
            this.printLine(`Balance: <span class="yellow">0.0 ETH</span>`);
            this.printLine('Nonce: 0');
            this.printLine('Type: EOA');
            this.printLine('Created At: 2023-06-15 14:32:45 UTC');
          }, 500);
          break;
        case 'list':
          const limit = args[1] ? parseInt(args[1]) : 10;
          const offset = args[2] ? parseInt(args[2]) : 0;
          this.printLine(`Listing accounts (limit: ${limit}, offset: ${offset})...`);
          // Simulate API call
          setTimeout(() => {
            this.printLine('<span class="cyan bold">Address</span>                                      <span class="cyan bold">Balance (ETH)</span>  <span class="cyan bold">Nonce</span>  <span class="cyan bold">Type</span>');
            this.printLine('─────────────────────────────────────────  ────────────  ─────  ────');
            for (let i = 0; i < 5; i++) {
              const addr = `0x${Math.random().toString(16).substr(2, 40)}`;
              const balance = (Math.random() * 100).toFixed(2);
              const nonce = Math.floor(Math.random() * 10);
              this.printLine(`${addr}  ${balance.padStart(12)}  ${nonce.toString().padStart(5)}  EOA`);
            }
          }, 500);
          break;
        case 'use':
          if (args.length < 2) {
            this.printLine('Usage: account use <address>');
            return;
          }
          const useAddress = args[1];
          this.currentAccount = useAddress;
          this.printLine(`Current account set to: <span class="green">${useAddress}</span>`);
          break;
        default:
          this.printLine(`Unknown account subcommand: ${subcommand}`);
          this.showHelp('account');
      }
    },
    
    handleBlockCommand(args) {
      if (args.length === 0) {
        this.showHelp('block');
        return;
      }
      
      const subcommand = args[0];
      
      switch (subcommand) {
        case 'get':
          if (args.length < 2) {
            this.printLine('Usage: block get <id>');
            return;
          }
          const id = args[1];
          this.printLine(`Getting block ${id}...`);
          // Simulate API call
          setTimeout(() => {
            this.printLine(`Block #<span class="yellow">${id}</span>`);
            this.printLine(`Hash: <span class="cyan">0x${Math.random().toString(16).substr(2, 64)}</span>`);
            this.printLine(`Parent Hash: 0x${Math.random().toString(16).substr(2, 64)}`);
            this.printLine('Timestamp: 2023-06-15 14:32:45 UTC');
            this.printLine(`Miner: 0x${Math.random().toString(16).substr(2, 40)}`);
            this.printLine('Size: 45.3 KB');
            this.printLine('Gas Used: 8,543,210');
            this.printLine('Gas Limit: 30,000,000');
            this.printLine('Transactions: 156');
          }, 500);
          break;
        case 'latest':
          this.printLine('Getting latest block...');
          // Simulate API call
          setTimeout(() => {
            const blockNumber = this.networkStatus.currentBlock;
            this.printLine(`Block #<span class="yellow">${blockNumber}</span>`);
            this.printLine(`Hash: <span class="cyan">0x${Math.random().toString(16).substr(2, 64)}</span>`);
            this.printLine(`Parent Hash: 0x${Math.random().toString(16).substr(2, 64)}`);
            this.printLine('Timestamp: 2023-06-15 14:32:45 UTC');
            this.printLine(`Miner: 0x${Math.random().toString(16).substr(2, 40)}`);
            this.printLine('Size: 45.3 KB');
            this.printLine('Gas Used: 8,543,210');
            this.printLine('Gas Limit: 30,000,000');
            this.printLine('Transactions: 156');
          }, 500);
          break;
        case 'list':
          const limit = args[1] ? parseInt(args[1]) : 10;
          const offset = args[2] ? parseInt(args[2]) : 0;
          this.printLine(`Listing blocks (limit: ${limit}, offset: ${offset})...`);
          // Simulate API call
          setTimeout(() => {
            this.printLine('<span class="cyan bold">Number</span>  <span class="cyan bold">Hash</span>                  <span class="cyan bold">Time</span>                    <span class="cyan bold">Txs</span>  <span class="cyan bold">Size</span>      <span class="cyan bold">Gas Used</span>');
            this.printLine('──────  ────────────────  ─────────────────────  ───  ────────  ─────────');
            const baseBlock = this.networkStatus.currentBlock;
            for (let i = 0; i < limit; i++) {
              const blockNum = baseBlock - offset - i;
              const hash = `0x${Math.random().toString(16).substr(2, 10)}`;
              const time = '2023-06-15 14:32:45';
              const txs = Math.floor(Math.random() * 200);
              const size = `${Math.floor(Math.random() * 100)}.${Math.floor(Math.random() * 10)} KB`;
              const gasUsed = Math.floor(Math.random() * 10000000);
              this.printLine(`${blockNum.toString().padStart(6)}  ${hash.padEnd(20)}  ${time}  ${txs.toString().padStart(3)}  ${size.padStart(8)}  ${gasUsed.toLocaleString()}`);
            }
          }, 500);
          break;
        default:
          this.printLine(`Unknown block subcommand: ${subcommand}`);
          this.showHelp('block');
      }
    },
    
    // Other command handlers would be implemented similarly
    handleContractCommand(args) {
      this.printLine('Contract commands not yet implemented in the web terminal.');
      this.showHelp('contract');
    },
    
    handleNetworkCommand(args) {
      if (args.length === 0 || args[0] === 'status') {
        this.printNetworkStatus();
      } else {
        this.printLine('Network commands not fully implemented in the web terminal.');
        this.showHelp('network');
      }
    },
    
    handleTokenCommand(args) {
      this.printLine('Token commands not yet implemented in the web terminal.');
      this.showHelp('token');
    },
    
    handleTxCommand(args) {
      this.printLine('Transaction commands not yet implemented in the web terminal.');
      this.showHelp('tx');
    },
    
    handleSystemCommand(args) {
      if (args.length === 0 || args[0] === 'stats') {
        this.printNodeStats();
      } else {
        this.printLine('System commands not fully implemented in the web terminal.');
        this.showHelp('system');
      }
    },
    
    handleConfigCommand(args) {
      this.printLine('Config commands not yet implemented in the web terminal.');
      this.showHelp('config');
    },
    
    handleDebugCommand(args) {
      this.printLine('Debug commands not yet implemented in the web terminal.');
      this.showHelp('debug');
    },
    
    printLine(text) {
      this.outputLines.push(text);
      this.$nextTick(() => {
        this.scrollToBottom();
      });
    },
    
    clearTerminal() {
      this.outputLines = [];
      this.showLogo = false;
    },
    
    scrollToBottom() {
      const terminal = this.$refs.terminalOutput;
      terminal.scrollTop = terminal.scrollHeight;
    },
    
    focusInput() {
      this.$refs.terminalInput.focus();
    },
    
    navigateHistory(direction) {
      if (this.commandHistory.length === 0) return;
      
      this.historyIndex += direction;
      
      if (this.historyIndex < 0) {
        this.historyIndex = 0;
      } else if (this.historyIndex >= this.commandHistory.length) {
        this.historyIndex = this.commandHistory.length;
        this.inputText = '';
        return;
      }
      
      this.inputText = this.commandHistory[this.historyIndex];
    },
    
    autocomplete() {
      // Simple autocomplete implementation
      const commands = [
        'help', 'clear', 'exit', 'quit',
        'account', 'block', 'contract', 'network',
        'token', 'tx', 'system', 'config', 'debug'
      ];
      
      const input = this.inputText.trim();
      if (!input) return;
      
      const matchingCommands = commands.filter(cmd => cmd.startsWith(input));
      
      if (matchingCommands.length === 1) {
        this.inputText = matchingCommands[0];
      } else if (matchingCommands.length > 1) {
        this.printLine(`\n${matchingCommands.join('  ')}`);
        this.printLine(`${this.prompt}${input}`);
      }
    },
    
    toggleFullscreen() {
      this.isFullscreen = !this.isFullscreen;
      if (this.isFullscreen) {
        document.documentElement.style.overflow = 'hidden';
        this.$el.classList.add('fullscreen');
      } else {
        document.documentElement.style.overflow = '';
        this.$el.classList.remove('fullscreen');
      }
      this.$nextTick(() => {
        this.focusInput();
      });
    }
  }
};
</script>

<style scoped>
.terminal-container {
  width: 100%;
  height: 600px;
  background-color: #1e1e1e;
  color: #f0f0f0;
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  font-family: 'Courier New', monospace;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.terminal-container.fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
  border-radius: 0;
}

.terminal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background-color: #333;
  border-bottom: 1px solid #444;
}

.terminal-title {
  font-weight: bold;
}

.terminal-controls {
  display: flex;
  gap: 8px;
}

.terminal-button {
  background-color: #555;
  border: none;
  color: #fff;
  padding: 4px 8px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.terminal-button:hover {
  background-color: #666;
}

.terminal-output {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  white-space: pre-wrap;
  word-break: break-word;
}

.terminal-input-container {
  display: flex;
  padding: 8px 16px;
  background-color: #252525;
  border-top: 1px solid #444;
}

.terminal-prompt {
  color: #4caf50;
  margin-right: 8px;
}

.terminal-input {
  flex: 1;
  background-color: transparent;
  border: none;
  color: #f0f0f0;
  font-family: 'Courier New', monospace;
  font-size: 14px;
  outline: none;
}

.terminal-logo {
  margin-bottom: 16px;
}

/* Global styles for terminal text colors */
:deep(.cyan) {
  color: #00bcd4;
}

:deep(.green) {
  color: #4caf50;
}

:deep(.yellow) {
  color: #ffeb3b;
}

:deep(.red) {
  color: #f44336;
}

:deep(.bold) {
  font-weight: bold;
}
</style>