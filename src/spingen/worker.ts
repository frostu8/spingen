import { Spingen, Spray as WasmSpray, Skin as WasmSkin, GifOptions } from '../../spingen-lib/pkg/spingen';
import { SpingenWorker, Spray, Skin, SprayFn, SkinFn, SkinOptions } from './shared.ts';
import * as Comlink from 'comlink';

// Create a new Spingen instance to communicate to our image algorithms.
const spingen = new Spingen();

async function loadFile(
  file: File,
  sprayFn: SprayFn,
  skinFn: SkinFn,
) {
  // load all sprays from file
  await spingen.fetchAll(file, (spray: WasmSpray) => {
    // remove all WASM typedata so we don't share any WASM data to the main
    // thread
    sprayFn({
      id: spray.id,
      name: spray.name,
    });

    spray.free();
  }, (skin: WasmSkin) => {
    // remove all WASM typedata so we don't share any WASM data to the main
    // thread
    skinFn({
      name: skin.name,
      realname: skin.realname,
      kartspeed: skin.kartspeed,
      kartweight: skin.kartweight,
      sprites: new Map(
        skin
          .sprites()
          .map((name) => {
            return [name, {
              frames: skin.frames(name),
            }];
          })
      ),
    });

    skin.free();
  });
}

function createSprayImage(spray: Spray) {
  return spingen.generateSprayImage(spray.id);
}

function createSkinAnimation(skin: Skin, spray: Spray | null, options: SkinOptions) {
  // build options struct
  const gifOptions = new GifOptions();
  gifOptions.scale = options.scale;

  // generate image
  return spingen.generateSkinAnimation(
    skin.name,
    spray?.id,
    options.sprite,
    options.frame,
    gifOptions,
  );
}

function createSkinThumbnail(skin: Skin, spray: Spray | null) {
  return spingen.generateSkinThumbnail(skin.name, spray?.id);
}

// Create comlink
const spingenWorker: SpingenWorker = {
  loadFile,
  createSprayImage,
  createSkinAnimation,
  createSkinThumbnail,
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
