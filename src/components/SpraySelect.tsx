import { SpingenContext, Spray } from '../spingen';
import { Switch, Match, For, createResource, useContext } from 'solid-js';

export interface SpraySelectProps {
  sprays: () => Spray[];
};

interface SpraySelectOptionProps {
  spray: Spray;
  selected: boolean;
}

const SpraySelectOption = (props: SpraySelectOptionProps) => {
  const buttonClass = props.selected ? "spray-button selected" : "spray-button";

  const spingen = useContext(SpingenContext);

  const [sprayUrl] = createResource(
    () => props.spray,
    async (spray: Spray) => {
      const url = await spingen?.createSprayImage(spray);
      return url;
    },
  );

  return (
    <button class={buttonClass}>
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
          <SpraySelectOption spray={item} selected={false}/>
        )}
      </For>
    </div>
  );
};

export default SpraySelect;
