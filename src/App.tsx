import { createSignal, createEffect } from 'solid-js'

import Header from './components/Header.tsx';
import SpraySelect from './components/SpraySelect.tsx';

import Worker from './worker/index.ts?worker';
import { RecvEvent, Spray } from './types.ts';

function App() {
  // setup lists
  const [sprays, setSprays] = createSignal([] as Spray[]);
  const [skins, setSkins] = createSignal([]);

  // create app web worker and register events
  const worker = new Worker();
  worker.onmessage = (msg: MessageEvent) => {
    const data = msg.data as RecvEvent;

    if (data.id === "newSpray") {
      setSprays((sprays) => sprays.concat([data.data]));
    }
  };

  const onFile = (file: File) => {
    worker.postMessage({ id: "newFile", data: file });
  };

  createEffect(() => {
    console.log(sprays());
  });

  return (
    <>
      <Header onFile={onFile}/>
      <main>
        <section class="select-menu">
          <SpraySelect sprays={sprays} />
        </section>
      </main>
    </>
  )
}

export default App
