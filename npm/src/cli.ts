#!/usr/bin/env node

import * as path from 'path';
import { spawn } from 'child_process';
import * as os from 'os';

const BIN_DIR = path.join(__dirname, '..', 'bin');
const BINARY_NAME = os.platform() === 'win32' ? 'pm.exe' : 'pm';
const BINARY_PATH = path.join(BIN_DIR, BINARY_NAME);

// Pass all arguments to the pm binary
const args = process.argv.slice(2);

const child = spawn(BINARY_PATH, args, {
  stdio: 'inherit', // Inherit stdin, stdout, stderr
});

child.on('error', (err) => {
  console.error(`Failed to start pm binary: ${err.message}`);
  process.exit(1);
});

child.on('close', (code) => {
  if (code !== null) {
    process.exit(code);
  } else {
    // Process was terminated by signal
    process.exit(1); 
  }
});
