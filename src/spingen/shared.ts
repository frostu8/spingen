// Shared types for worker-main thread communication.
export type SprayFn = (spray: Spray) => void;
export type SkinFn = (spray: Skin) => void;

export interface SpingenWorker {
  loadFile: (file: File, sprayFn: SprayFn, skinFn: SkinFn) => Promise<void>;
  createSprayImage: (spray: Spray) => string;
}

export type Spray = {
  id: string,
  name: string,
};

export type Skin = {
  name: string,
  realname: string,
  kartspeed: number,
  kartweight: number,
};
