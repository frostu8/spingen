import { Spingen, Spray as WasmSpray } from '../../spingen-lib/pkg/spingen';
import { SpingenWorker, Spray, SprayFn, SkinFn } from './shared.ts';
import * as Comlink from 'comlink';

// Create a new Spingen instance to communicate to our image algorithms.
const spingen = new Spingen();

async function loadFile(
  file: File,
  sprayFn: SprayFn,
  _skinFn: SkinFn,
) {
  // load all sprays from file
  await spingen.fetchSprays(file, (spray: WasmSpray) => {
    // remove all WASM typedata so we don't share any WASM data to the main
    // thread
    sprayFn({
      id: spray.id,
      name: spray.name,
    });

    spray.free();
  });

  // load all skins from file
  //await spingen.fetchSprays(file, sprayFn);
}

function createSprayImage(spray: Spray) {
  return spingen.generateSprayImage(spray.id);
}

// Create comlink
const spingenWorker: SpingenWorker = {
  loadFile,
  createSprayImage,
};

// Expose comlink
Comlink.expose(spingenWorker);

// load default sprays
const sprays = spingen
  .fetchDefaultSprays()
  .map((spray) => {
    const newSpray = {
      id: spray.id,
      name: spray.name,
    };
    spray.free();

    return newSpray;
  });

console.log('WASM initialized, sending ready');
self.postMessage({ id: 'READY', sprays });
