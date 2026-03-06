#!/usr/bin/env node

const os = require('os');
const fs = require('fs');
const path = require('path');

const platform = os.platform();
const arch = os.arch();

if (platform !== 'linux') {
  console.error(`\x1b[35m╔══════════════════════════════════════════════╗\x1b[0m`);
  console.error(`\x1b[35m║\x1b[0m  GoonSharp only supports \x1b[36mLinux x64\x1b[0m for now.  \x1b[35m║\x1b[0m`);
  console.error(`\x1b[35m║\x1b[0m  Detected: ${(platform + ' ' + arch).padEnd(31)} \x1b[35m║\x1b[0m`);
  console.error(`\x1b[35m║\x1b[0m  macOS/Windows support coming soon™          \x1b[35m║\x1b[0m`);
  console.error(`\x1b[35m╚══════════════════════════════════════════════╝\x1b[0m`);
  process.exit(1);
}

if (arch !== 'x64') {
  console.error(`\x1b[35m[goonsharp]\x1b[0m Unsupported architecture: ${arch}. Only x64 is supported.`);
  process.exit(1);
}

// Ensure binaries are executable
const binDir = path.join(__dirname, 'bin');
const binaries = ['goonsharp', 'goonhub'];

for (const bin of binaries) {
  const binPath = path.join(binDir, bin);
  if (fs.existsSync(binPath)) {
    try {
      fs.chmodSync(binPath, 0o755);
    } catch (e) {
      // Might fail on some systems, that's OK
    }
  } else {
    console.error(`\x1b[31m[goonsharp]\x1b[0m Binary not found: ${binPath}`);
    process.exit(1);
  }
}

console.log(`\x1b[35m╔══════════════════════════════════════════════╗\x1b[0m`);
console.log(`\x1b[35m║\x1b[0m                                              \x1b[35m║\x1b[0m`);
console.log(`\x1b[35m║\x1b[0m   \x1b[1m\x1b[36mGoonSharp v69.0.0\x1b[0m installed successfully!  \x1b[35m║\x1b[0m`);
console.log(`\x1b[35m║\x1b[0m                                              \x1b[35m║\x1b[0m`);
console.log(`\x1b[35m║\x1b[0m   Try it:  \x1b[33mgoonsharp hello.goons\x1b[0m            \x1b[35m║\x1b[0m`);
console.log(`\x1b[35m║\x1b[0m   Docs:    \x1b[34mhttps://goonsharp.dev\x1b[0m            \x1b[35m║\x1b[0m`);
console.log(`\x1b[35m║\x1b[0m                                              \x1b[35m║\x1b[0m`);
console.log(`\x1b[35m╚══════════════════════════════════════════════╝\x1b[0m`);
