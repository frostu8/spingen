import { SpingenContext, Spray } from '../spingen';
import { Switch, Match, For, createResource, useContext } from 'solid-js';

export interface SpraySelectProps {
  sprays: () => Spray[];
  value?: () => Spray | undefined;
  onChange: (spray: Spray) => void;
};

interface SpraySelectOptionProps {
  spray: Spray;
  selected: () => boolean;
  onClick: (ev: MouseEvent) => void;
}

const SpraySelectOption = (props: SpraySelectOptionProps) => {
  const spingen = useContext(SpingenContext);

  const [sprayUrl] = createResource(
    () => props.spray,
    async (spray: Spray) => {
      const url = await spingen?.createSprayImage(spray);
      return url;
    },
  );

  return (
    <button
      class={
        props.selected() ? "spray-button selected" : "spray-button"
      }
      on:click={props.onClick}
    >
      <Switch>
        <Match when={sprayUrl()}>
          <img src={sprayUrl()} alt="spray can"/>
        </Match>
      </Switch>
      { props.spray.name }
    </button>
  );
}

const SpraySelect = (props: SpraySelectProps) => {
  return (
    <div class="spray-select">
      <For each={props.sprays()}>
        {(item, _index) => (
          <SpraySelectOption
            spray={item}
            selected={() => props.value ? item.id === props.value()?.id : false}
            onClick={() => props.onChange(item)}
          />
        )}
      </For>
    </div>
  );
};

export default SpraySelect;
