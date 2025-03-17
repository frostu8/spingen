import { createSignal, createEffect } from 'solid-js'

import Header from './components/Header.tsx';
import SpraySelect from './components/SpraySelect.tsx';

import { Spingen, Spray, SpingenContext } from './spingen';

function App() {
  // setup lists
  const [sprays, setSprays] = createSignal([] as Spray[]);
  const [skins, setSkins] = createSignal([]);

  // create spingen
  const spingen = new Spingen();

  // setup events
  spingen.onSpray = (spray: Spray) => {
    setSprays((sprays) => sprays.concat([spray]));
  };

  const onFile = (file: File) => {
    spingen.loadFile(file);
  };

  createEffect(() => {
    console.log(sprays());
  });

  return (
    <SpingenContext.Provider value={spingen}>
      <Header onFile={onFile}/>
      <main>
        <section class="select-menu">
          <SpraySelect sprays={sprays} />
        </section>
      </main>
    </SpingenContext.Provider>
  )
}

export default App
