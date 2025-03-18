import { createResource, For, Switch, Match, useContext, createMemo } from 'solid-js';
import { A } from '@solidjs/router';

import { Skin, Spray, SpingenContext } from '../spingen';

export type SkinWithSpray = Skin & {
  spray: () => Spray | undefined | null;
  setSpray: (spray: Spray | null) => void;
};

export interface SkinSelectProps {
  skins: () => SkinWithSpray[];
};

interface SkinButtonProps {
  skin: SkinWithSpray;
}

const SkinButton = (props: SkinButtonProps) => {
  const spingen = useContext(SpingenContext);

  const [thumbnailBlob] = createResource(
    () => {
      const {spray, setSpray: __, ...skin} = props.skin;
      return {
        spray: spray() ?? null,
        skin,
      };
    },
    async (data) => {
      const url = await spingen?.createSkinThumbnail(data.skin, data.spray);
      return url;
    },
  )

  const thumbnailUrl = createMemo<string | undefined>((oldSrc) => {
    if (oldSrc) {
      // cleanup old src
      URL.revokeObjectURL(oldSrc);
    }
    const thumbnail = thumbnailBlob();
    return thumbnail;
  });

  return (
    <A
      class="skin-button btn"
      href={`?char=${props.skin.name}`}
    >
      <Switch>
        <Match when={thumbnailUrl()}>
          <img src={thumbnailUrl()} />
        </Match>
      </Switch>
      <p>
        {props.skin.realname.replace(/_/g, ' ')}
      </p>
    </A>
  );
}

const SkinSelect = (props: SkinSelectProps) => {
  return (
    <section class="skin-container">
      <For
        each={props.skins()}
      >
        {(item, _index) => (
          <SkinButton skin={item}/>
        )}
      </For>
    </section>
  );
};

export default SkinSelect;
