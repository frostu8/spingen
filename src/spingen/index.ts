// All of the internal stuff for spingen.
import * as Comlink from 'comlink';
import { createContext } from 'solid-js';
import { Spray, Skin, SpingenWorker, SkinOptions } from './shared.ts';

// share all types
export * from './shared.ts';

export class Spingen {
  comlink: Comlink.Remote<SpingenWorker>;

  onReady: (sprays: Spray[]) => void;
  onSpray: (spray: Spray) => void;
  onSkin: (skin: Skin) => void;

  constructor() {
    // In development mode, `import`s in workers are not transformed, so you
    // must use `{ type: "module" }`.
    const worker = import.meta.env.DEV
      ? new Worker(new URL("./worker.ts", import.meta.url), { type: "module", name: "spingen" })
        // In build mode, let Vite and vite-plugin-top-level-await build a single-file
        // bundle of your worker that works on both modern browsers and Firefox.
      : new Worker(new URL("./worker.ts", import.meta.url), { type: "classic", name: "spingen" });

    this.comlink = Comlink.wrap(worker);

    // initialize default handlers
    this.onReady = () => {};
    this.onSpray = () => {};
    this.onSkin = () => {};

    // wait for init
    const readyListener = (msg: MessageEvent) => {
      if (msg.data && msg.data.id === "READY") {
        this.onReady(msg.data.sprays);
        worker.removeEventListener("message", readyListener);
      }
    };

    worker.addEventListener("message", readyListener);
  }

  loadFile(file: File): Promise<void> {
    return this.comlink.loadFile(file, Comlink.proxy(this.onSpray), Comlink.proxy(this.onSkin));
  }

  createSprayImage(spray: Spray): Promise<string> {
    return this.comlink.createSprayImage(spray);
  }

  createSkinAnimation(skin: Skin, spray: Spray | null, options: SkinOptions): Promise<string> {
    return this.comlink.createSkinAnimation(skin, spray, options);
  }

  createSkinThumbnail(skin: Skin, spray: Spray | null): Promise<string> {
    return this.comlink.createSkinThumbnail(skin, spray);
  }
}

// Create a context provider for the spingen
export const SpingenContext = createContext<Spingen>();
