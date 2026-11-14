import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
const VERSION_S_S_M_REGEX = /^version\s*=\s*"[^"]+"/m;

const root = resolve(import.meta.dirname, '..');
const packageVersion = JSON.parse(readFileSync(resolve(root, 'package.json'), 'utf8')).version;
if (!/^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/.test(packageVersion)) throw new Error(`Invalid release version: ${packageVersion}`);
const manifestPath = resolve(root, 'Cargo.toml');
const manifest = readFileSync(manifestPath, 'utf8');
const packageStart = manifest.indexOf('[package]');
if (packageStart === -1) throw new Error('Cargo package section not found');
const nextSection = manifest.indexOf('\n[', packageStart + '[package]'.length);
const packageEnd = nextSection === -1 ? manifest.length : nextSection;
const packageSection = manifest.slice(packageStart, packageEnd);
if (!VERSION_S_S_M_REGEX.test(packageSection)) throw new Error('Cargo package version not found');
const updated = manifest.slice(0, packageStart) + packageSection.replace(VERSION_S_S_M_REGEX, `version = "${packageVersion}"`) + manifest.slice(packageEnd);
if (updated !== manifest) writeFileSync(manifestPath, updated);
