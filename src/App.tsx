import { createSignal } from 'solid-js'
import { useSearchParams, Router, Route } from '@solidjs/router';

import Header from './components/Header.tsx';
import SpraySelect from './components/SpraySelect.tsx';
import ViewSkin from './components/ViewSkin.tsx';
import SkinSelect, { SkinWithSpray } from './components/SkinSelect.tsx';
import { Toaster, toast } from 'solid-toast';

import { Spingen, Spray, Skin, SpingenContext } from './spingen';

interface ShowProps {
  skins: () => SkinWithSpray[],
  sprays: () => Spray[],
}

const Show = (props: ShowProps) => {
  const [params, _setParams] = useSearchParams();

  const currentSkin = () => {
    return props.skins().find((skin) => skin.name === params.char);
  };

  return (
    <main>
      <section class="select-menu">
        <SkinSelect skins={props.skins} />
        <SpraySelect
          sprays={props.sprays}
          value={currentSkin()?.spray}
          onChange={(spray) => {
            const skin = currentSkin();
            if (skin) {
              skin.setSpray(spray);
            }
          }}
        />
      </section>
      <ViewSkin skin={currentSkin} />
    </main>
  );
}

function App() {
  // setup lists
  const [sprays, setSprays] = createSignal<Spray[]>([]);
  const [skins, setSkins] = createSignal<SkinWithSpray[]>([]);

  // create spingen
  const spingen = new Spingen();

  // setup events
  spingen.onSpray = (spray: Spray) => {
    setSprays((sprays) => sprays.concat([spray]));
  };
  spingen.onSkin = (skin: Skin) => {
    setSkins((skins) => {
      const [spray, setSpray] = createSignal<Spray>();

      return skins.concat([{
        spray,
        setSpray,
        ...skin
      }]);
    });
  };
  spingen.onReady = (otherSprays: Spray[]) => {
    setSprays((sprays) => sprays.concat(otherSprays));
  }

  // reactive dom events
  const onFile = (file: File) => {
    // setup toaster promise
    toast.promise(
      spingen.loadFile(file),
      {
        loading: `Loading file ${file.name}`,
        success: `Loaded file ${file.name}`,
        error: (err) => `Failed to load file: ${err}`,
      }
    );
  };

  return (
    <SpingenContext.Provider value={spingen}>
      <Toaster
        position="bottom-left"
        gutter={8}
      />
      <Header onFile={onFile}/>
      <Router>
        <Route path="/" component={() => <Show skins={skins} sprays={sprays} />}/>
      </Router>
    </SpingenContext.Provider>
  )
}

export default App
