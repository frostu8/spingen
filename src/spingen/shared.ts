// Shared types for worker-main thread communication.
export type SprayFn = (spray: Spray) => void;
export type SkinFn = (spray: Skin) => void;

export interface SpingenWorker {
  loadFile: (file: File, sprayFn: SprayFn, skinFn: SkinFn) => Promise<void>;
  createSprayImage: (spray: Spray) => string;
  createSkinAnimation: (skin: Skin, spray: Spray | null, options: SkinOptions) => string;
  createSkinThumbnail: (skin: Skin, spray: Spray | null) => string;
}

export interface SkinOptions {
  sprite: string;
  frame: string;
  scale: number;
};

export interface Spray {
  id: string;
  name: string;
};

export interface Sprite {
  frames: string[];
}

export interface Skin {
  name: string;
  realname: string;
  kartspeed: number;
  kartweight: number;
  sprites: Map<string, Sprite>;
};
