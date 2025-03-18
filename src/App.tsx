import { createSignal, createEffect } from 'solid-js'

import Header from './components/Header.tsx';
import SpraySelect from './components/SpraySelect.tsx';

import { Spingen, Spray, Skin, SpingenContext } from './spingen';

function App() {
  // setup lists
  const [sprays, setSprays] = createSignal<Spray[]>([]);
  const [skins, setSkins] = createSignal<Skin[]>([]);

  // create spingen
  const spingen = new Spingen();

  // setup events
  spingen.onSpray = (spray: Spray) => {
    setSprays((sprays) => sprays.concat([spray]));
  };
  spingen.onSkin = (skin: Skin) => {
    setSkins((skins) => skins.concat([skin]));
  };
  spingen.onReady = (otherSprays: Spray[]) => {
    setSprays((sprays) => sprays.concat(otherSprays));
  }

  // reactive dom events
  const onFile = (file: File) => {
    spingen.loadFile(file)
      .then(() => console.log("loaded file", file.name))
      .catch((error) => console.error(error));
  };

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
