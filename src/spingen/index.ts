// All of the internal stuff for spingen.
import Worker from './worker.ts?worker';
import { createContext } from 'solid-js';
import { Spray, RecvEvent } from './shared.ts';

// share all types
export * from './shared.ts';

interface Request {
  resolve: (data: any) => void;
  reject: (data: any) => void;
  seq: number;
}

export class Spingen {
  worker: Worker;
  seq: number;

  onSpray?: (spray: Spray) => void;

  requests: Request[];

  constructor() {
    this.worker = new Worker({ name: "spingen" });
    this.seq = 0;
    this.requests = [];

    this.worker.onmessage = (msg: MessageEvent) => {
      const data = msg.data as RecvEvent;

      if (data.id === "newSpray") {
        if (this.onSpray) {
          this.onSpray(data.data);
        }
      } else if (data.id === "generateSprayImage") {
        const req = this.getRequest(data.seq);
        if (req) {
          req.resolve(data.data);
        }
      } else if (data.id === "error") {
        const req = this.getRequest(data.seq);
        if (req) {
          req.reject(data.data);
        }
      }
    };
  }

  getRequest(seq: number): Request | undefined {
    return this.requests.find((req) => req.seq === seq);
  }

  loadFile(file: File) {
    this.worker.postMessage({ id: "newFile", data: file });
  }

  async createSprayImage(spray: Spray): Promise<string> {
    this.seq += 1;

    this.worker.postMessage({
      id: "generateSprayImage",
      seq: this.seq,
      data: spray.id,
    });
    return new Promise((resolve, reject) => {
      this.requests.push({ seq: this.seq, resolve, reject });
    });
  }
}

// Create a context provider for the spingen
export const SpingenContext = createContext<Spingen>();
