import { Spingen } from '../../spingen-lib/pkg/spingen';
import { SendEvent } from '../types.ts';

// Create a new Spingen instance to communicate to our image algorithms.
const spingen = new Spingen();

self.onmessage = async (ev: MessageEvent) => {
  const data = ev.data as SendEvent;

  if (data.id === "newFile") {
    console.log("loading file " + data.data.name);

    // load all sprays from file
    await spingen.fetchSprays(data.data, (spray: any) => {
      // post new spray for each new spray
      self.postMessage({
        id: "newSpray",
        data: {
          id: spray.id,
          name: spray.name,
        },
      })
    })
  }
};
